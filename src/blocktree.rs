use std::collections::hash_map::{self, HashMap};

use protobuf::RepeatedField;

use balance::{BalancePatchExt, LedgerReader, LedgerWriter, LedgerSnapshot,
              Patchable};
use block::{BlockWithDiffExt, HashedBlockExt};
use crypto::{HashDigest, PublicKey};
use error::{SimplesError, SimplesResult};
use simples_pb::{Balance, BalancePatch, BlockWithDiff, HashedBlock};
use store::{MessageStore, KeyValueStore, ProtobufStore};

const GENESIS_FIELD: &'static str = "meta:genesis";
const HEAD_FIELD: &'static str = "meta:head";

fn make_genesis_block_diff(genesis: HashedBlock)
                           -> SimplesResult<BlockWithDiff>
{
    try!(genesis.verify());
    let num_tx = genesis.get_block().get_transactions().len();
    if num_tx == 0 {
        Ok(BlockWithDiff::new())
    } else if num_tx == 1 {
        let mut cache = HashMap::<PublicKey, u64>::new();
        let commit =
            genesis.get_block().get_transactions()[0].get_commit().clone();
        for transfer in commit.get_transfers() {
            let destination =
                try!(PublicKey::from_bytes(transfer.get_destination_pk()));
            match cache.entry(destination) {
                hash_map::Entry::Occupied(mut tokens) => {
                    let new_balance = tokens.get() + transfer.get_tokens();
                    tokens.insert(new_balance);
                },
                hash_map::Entry::Vacant(tokens) => {
                    tokens.insert(transfer.get_tokens());
                }
            };
        }
        let mut block_diff = BlockWithDiff::new();
        block_diff.set_hashed_block(genesis);
        for (address, tokens) in cache.into_iter() {
            let mut before = Balance::new();
            before.set_tokens(0u64);
            before.set_op_index(0u32);

            let mut after = Balance::new();
            after.set_tokens(tokens);
            after.set_op_index(0u32);

            let mut patch = BalancePatch::new();
            patch.set_public_key(address.0.to_vec());
            patch.set_before(before);
            patch.set_after(after);
            block_diff.mut_diff().push(patch);
        }
        Ok(block_diff)
    } else {
         Err(SimplesError::new(
             "Genesis block must contain at most 1 transaction."))
    }
}

fn format_balance_key(address: &PublicKey) -> String {
    format!("a:{}", address)
}

fn format_block_key(block_hash: &HashDigest) -> String {
    format!("b:{}", block_hash)
}

pub struct BlockTreeStore<Store: KeyValueStore> {
    store: MessageStore<Store>,
}

impl<Store: KeyValueStore> BlockTreeStore<Store> {
    pub fn new(kv_store: Store, new_genesis: Option<HashedBlock>)
               -> SimplesResult<Self>
    {
        let mut store = MessageStore::<Store>::new(kv_store);
        let mut blocktree;
        let maybe_genesis_hash =
            match try!(store.get_bytes(GENESIS_FIELD.as_bytes())) {
                Some(hash_raw) => Some(try!(HashDigest::from_bytes(&hash_raw))),
                None => None
            };
        match (maybe_genesis_hash, new_genesis) {
            (Some(genesis_hash), Some(new_genesis_block)) => {
                let new_genesis_hash = try!(new_genesis_block.decode_hash());
                if new_genesis_hash != genesis_hash {
                    return Err(SimplesError::new(&format!(
                        "Blocktree already has genesis {} != {} (requested)",
                        genesis_hash, new_genesis_hash)));
                }
                println!("Blocktree already has genesis {}.", genesis_hash);
                blocktree = BlockTreeStore { store: store }
            },
            (Some(genesis_hash), None) => {
                println!("Blocktree has genesis {}.", genesis_hash);
                blocktree = BlockTreeStore { store: store }
            },
            (None, Some(new_genesis_block)) => {
                let new_genesis_hash = try!(new_genesis_block.decode_hash());
                println!("Store doesn't have a genesis block. Setting it to {}.",
                         new_genesis_hash);
                let genesis = try!(make_genesis_block_diff(new_genesis_block));
                let genesis_key = format_block_key(&new_genesis_hash);
                try!(store.set_bytes(
                    GENESIS_FIELD.as_bytes(), &new_genesis_hash.0));
                try!(store.set_bytes(
                    HEAD_FIELD.as_bytes(), &new_genesis_hash.0));
                try!(store.set_message(genesis_key.as_bytes(), &genesis));
                blocktree = BlockTreeStore { store: store };
                for patch in genesis.get_diff().iter() {
                    try!(blocktree.apply_patch(patch.clone()));
                }
            },
            (None, None) => return Err(SimplesError::new(
                "No genesis block was specified and store doesn't contain one."))
        }
        Ok(blocktree)
    }

    pub fn get_head(&self) -> SimplesResult<HashedBlock> {
        let head_hash = try!(self.get_head_hash());
        let head = try!(self.get_block(&head_hash)).expect(
            "FATAL: Corrupted blocktree, head block missing from kv-store.");
        Ok(head)
    }

    pub fn get_head_hash(&self) -> SimplesResult<HashDigest> {
        let head_hash_raw = try!(self.store.get_bytes(HEAD_FIELD.as_bytes()))
            .expect("FATAL: Corrupted blocktree, store is headless.");
        let maybe_head_hash = HashDigest::from_bytes(&head_hash_raw);
        assert!(maybe_head_hash.is_ok(),
                "FATAL: Corrupted blocktree, head block has invalid hash.");
        Ok(maybe_head_hash.unwrap())
    }

    pub fn set_head(&mut self, new_head_hash: &HashDigest) -> SimplesResult<()> {
        if *new_head_hash == try!(self.get_head_hash()) {
            return Ok(());
        }
        try!(try!(self.get_block(new_head_hash)).ok_or(
            SimplesError::new(&format!(
                "Tried to set head to {}, but it doesn't exist in the blocktree",
                new_head_hash))));
        let mut patches = try!(self.snapshot_at(new_head_hash)).make_patches();
        for patch in patches.drain() { try!(self.apply_patch(patch)); }
        self.store.set_bytes(HEAD_FIELD.as_bytes(), &new_head_hash.0)
    }

    pub fn get_genesis(&self) -> SimplesResult<HashedBlock> {
        let genesis_hash = try!(self.get_genesis_hash());
        let genesis = try!(self.get_block(&genesis_hash)).expect(
            "FATAL: Corrupted blocktree store, genesis block missing \
             from kv-store.");
        Ok(genesis)
    }

    pub fn get_genesis_hash(&self) -> SimplesResult<HashDigest> {
        let genesis_hash_raw =
            try!(self.store.get_bytes(GENESIS_FIELD.as_bytes())).expect(
                "FATAL: Corrupted blocktree, genesis hash key: {} is not set.");
        let maybe_genesis_hash = HashDigest::from_bytes(&genesis_hash_raw);
        assert!(maybe_genesis_hash.is_ok(),
                "FATAL: Corrupted blocktree, genesis block has invalid hash.");
        Ok(maybe_genesis_hash.unwrap())
    }

    pub fn get_block(&self, block_hash: &HashDigest)
                     -> SimplesResult<Option<HashedBlock>> {
        Ok(try!(self.get_block_diff(block_hash)).map(|mut x| x.take_hashed_block()))
    }

    pub fn insert_block(&mut self, block: HashedBlock) -> SimplesResult<()> {
        try!(block.verify());
        let block_height = block.get_block().get_height();
        let previous_hash = try!(block.decode_previous());
        let previous_block =
            try!(try!(self.get_block(&previous_hash)).ok_or(SimplesError::new(&format!(
            "insert error: previous block {} missing from kv-store", previous_hash))));
        let previous_height = previous_block.get_block().get_height();

        if previous_height + 1 != block_height {
            return Err(SimplesError::new(&format!(
                "insert error: invalid block height {} (expected {})",
                block_height, previous_height + 1)));
        }

        let mut block_diff = BlockWithDiff::new();
        {
            let mut snapshot = try!(self.snapshot_at(&previous_hash));
            snapshot.commit();
            try!(snapshot.apply_block(&block));
            block_diff.set_diff(RepeatedField::from_vec(snapshot.make_patches()));
        }
        block_diff.set_hashed_block(block);
        Ok(try!(self.set_block_diff(&block_diff)))
    }

    pub fn snapshot(&self) -> LedgerSnapshot<Self> { LedgerSnapshot::new(self) }

    pub fn snapshot_at(&self, block_hash: &HashDigest)
                       -> SimplesResult<LedgerSnapshot<Self>>
    {
        let mut snapshot = LedgerSnapshot::new(self);
        let mut fast_forward_stack = vec![];
        {
            let mut target_ancestor =
                try!(try!(self.get_block_diff(&block_hash))
                     .ok_or(SimplesError::new(&format!("snapshot_at error: patch \
                            missing from kv-store for block {}", block_hash))));
            let mut head_ancestor =
                try!(self.get_block_diff(&try!(self.get_head_hash()))).unwrap();
            let mut target_ancestor_hash = target_ancestor.decode_hash().unwrap();
            let mut head_ancestor_hash = head_ancestor.decode_hash().unwrap();

            let genesis = try!(self.get_genesis_hash());
            while target_ancestor_hash != head_ancestor_hash {
                if head_ancestor.get_height() >= target_ancestor.get_height() {
                    assert!(head_ancestor_hash != genesis);
                    for patch in head_ancestor.take_diff().into_iter() {
                        try!(snapshot.apply_patch(patch.reverse()));
                    }
                    head_ancestor_hash = try!(head_ancestor.decode_previous());
                    head_ancestor = try!(self.get_block_diff(&head_ancestor_hash)).expect(
                        "FATAL: Corrupted blocktree, missing blocks from the \
                         history of head");
                } else {
                    assert!(target_ancestor_hash != genesis);
                    target_ancestor_hash = try!(target_ancestor.decode_previous());
                    fast_forward_stack.push(target_ancestor);
                    target_ancestor =
                        try!(self.get_block_diff(&target_ancestor_hash))
                         .expect(&format!("FATAL: Corrupted blocktree, missing \
                          block from target history: {}", target_ancestor_hash));
                }
            }
        }
        loop { match fast_forward_stack.pop() {
            Some(mut block_diff) => {
                for patch in block_diff.take_diff().into_iter() {
                    try!(snapshot.apply_patch(patch));
                }
            },
            None => break
        } }
        Ok(snapshot)
    }

    fn get_block_diff(&self, block_hash: &HashDigest)
                      -> SimplesResult<Option<BlockWithDiff>> {
        let store_key = format_block_key(block_hash);
        self.store.get_message(store_key.as_bytes())
    }

    fn set_block_diff(&mut self, block_diff: &BlockWithDiff)
                      -> SimplesResult<()>  {
        let store_key = format_block_key(&try!(block_diff.decode_hash()));
        self.store.set_message(store_key.as_bytes(), block_diff)
    }
}

impl<Store: KeyValueStore> LedgerReader for BlockTreeStore<Store> {
    fn get_balance(&self, address: &PublicKey) -> SimplesResult<Balance> {
        let db_key = format_balance_key(address);
        match try!(self.store.get_message(db_key.as_bytes())) {
            Some(balance) => Ok(balance),
            None => {
                let mut balance = Balance::new();
                balance.set_tokens(0);
                balance.set_op_index(0);
                Ok(balance)
            }
        }
    }
}

impl<Store: KeyValueStore> LedgerWriter for BlockTreeStore<Store> {
    fn set_balance(&mut self, address: &PublicKey, balance: Balance)
                   -> SimplesResult<()> {
        let db_key = format_balance_key(address);
        self.store.set_message(db_key.as_bytes(), &balance)
    }
}

/*****  Tests  *****/

use block::GenesisBuilder;
use crypto::gen_keypair;

#[test]
fn test_make_genesis_block_diff_empty() {
    use block::GenesisBuilder;

    let builder = GenesisBuilder::new();
    let genesis = builder.build();
    let maybe_patch = make_genesis_block_diff(genesis);
    assert!(maybe_patch.is_ok());
    assert!(0 == maybe_patch.unwrap().get_diff().len());
}

#[test]
fn test_make_genesis_patch_non_unqiue_dest() {
    use std::iter::FromIterator;
    use block::GenesisBuilder;
    use crypto::gen_keypair;

    let (pk1, _) = gen_keypair();
    let (pk2, _) = gen_keypair();
    let mut builder = GenesisBuilder::new();
    builder.add_transfer(pk1.clone(), 101);
    builder.add_transfer(pk2.clone(), 271);
    builder.add_transfer(pk1.clone(), 5000);
    let genesis = builder.build();
    let maybe_diff = make_genesis_block_diff(genesis);
    assert!(maybe_diff.is_ok());
    assert!(2 == maybe_diff.as_ref().unwrap().get_diff().len());
    let patch_map: HashMap<PublicKey, BalancePatch> =
        FromIterator::from_iter(maybe_diff.unwrap().get_diff().iter().map(
            |patch| {(PublicKey::from_bytes(patch.get_public_key()).unwrap(),
                      patch.clone())}));

    let maybe_patch_pk1 = patch_map.get(&pk1);
    assert!(maybe_patch_pk1.is_some());
    let patch_pk1 = maybe_patch_pk1.unwrap();
    assert!(pk1 == PublicKey::from_bytes(patch_pk1.get_public_key()).unwrap());
    assert!(0 == patch_pk1.get_before().get_tokens());
    assert!(0 == patch_pk1.get_before().get_op_index());
    assert!(5101 == patch_pk1.get_after().get_tokens());
    assert!(0 == patch_pk1.get_after().get_op_index());

    let maybe_patch_pk2 = patch_map.get(&pk2);
    assert!(maybe_patch_pk2.is_some());
    let patch_pk2 = maybe_patch_pk2.unwrap();
    assert!(pk2 == PublicKey::from_bytes(patch_pk2.get_public_key()).unwrap());
    assert!(0 == patch_pk2.get_before().get_tokens());
    assert!(0 == patch_pk2.get_before().get_op_index());
    assert!(271 == patch_pk2.get_after().get_tokens());
    assert!(0 == patch_pk2.get_after().get_op_index());
}

#[test]
fn test_genesis_builder() {
    let (pk1, sk1) = gen_keypair();
    let (pk2, sk2) = gen_keypair();
    let (pk3, sk3) = gen_keypair();
    let mut builder = GenesisBuilder::new();
    builder.add_transfer(pk1, 10);
    builder.add_transfer(pk2, 22);
    builder.add_transfer(pk3, 107);
    let genesis = builder.build();

    assert!(genesis.verify().is_ok());
    let txs = genesis.get_block().get_transactions();
    assert!(1 == txs.len());
    assert!(3 == txs[0].get_commit().get_transfers().len());
}

#[test]
fn test_blocktree_new_no_genesis() {
    let kv_store1 = HashMap::<Vec<u8>, Vec<u8>>::new();
    let maybe_blocktree1 = BlockTreeStore::new(kv_store1, None);
    assert!(maybe_blocktree1.is_err());
}

#[test]
fn test_blocktree_sets_head_and_genesis() {
    let (pk1, sk1) = gen_keypair();
    let (pk2, sk2) = gen_keypair();
    let mut builder = GenesisBuilder::new();
    builder.add_transfer(pk1, 100);
    builder.add_transfer(pk2, 100);
    let genesis = builder.build();
    assert!(genesis.verify().is_ok());

    let kv_store = HashMap::<Vec<u8>, Vec<u8>>::new();
    let maybe_blocktree = BlockTreeStore::new(kv_store, Some(genesis.clone()));
    assert!(maybe_blocktree.is_ok());
    let blocktree = maybe_blocktree.unwrap();

    let maybe_genesis_hash = blocktree.store.get_bytes(
        GENESIS_FIELD.as_bytes()).unwrap();
    let maybe_head_hash = blocktree.store.get_bytes(
        HEAD_FIELD.as_bytes()).unwrap();
    assert!(maybe_genesis_hash.is_some());
    assert!(maybe_head_hash.is_some());

    let genesis_hash =
        HashDigest::from_bytes(&maybe_genesis_hash.unwrap()).unwrap();
    let head_hash =
        HashDigest::from_bytes(&maybe_head_hash.unwrap()).unwrap();
    assert!(genesis.decode_hash().unwrap() == genesis_hash);
    assert!(genesis_hash == head_hash);
    assert!(blocktree.get_genesis_hash().unwrap() == genesis_hash);
    assert!(blocktree.get_head_hash().unwrap() == genesis_hash);

    let maybe_genesis_block = blocktree.store.get_message(
        format_block_key(&genesis_hash).as_bytes()).unwrap();
    assert!(maybe_genesis_block.is_some());
    assert!(genesis == maybe_genesis_block.unwrap());
    assert!(genesis == blocktree.get_genesis().unwrap());

    let maybe_head_block = blocktree.store.get_message(
        format_block_key(&genesis_hash).as_bytes()).unwrap();
    assert!(maybe_head_block.is_some());
    assert!(genesis == maybe_head_block.unwrap());
    assert!(genesis == blocktree.get_head().unwrap());
}

#[test]
fn test_blocktree_insert_and_snapshot_at() {
    use tx::TransactionBuilder;

    let (pk1, sk1) = gen_keypair();
    let (pk2, sk2) = gen_keypair();
    let (pk3, sk3) = gen_keypair();
    let mut builder = GenesisBuilder::new();
    builder.add_transfer(pk1.clone(), 980);
    builder.add_transfer(pk2.clone(), 73);
    builder.add_transfer(pk3.clone(), 10);
    let genesis = builder.build();
    let genesis_hash = genesis.decode_hash().unwrap();
    genesis.verify().unwrap();

    let kv_store = HashMap::<Vec<u8>, Vec<u8>>::new();
    let mut blocktree = BlockTreeStore::new(kv_store, Some(genesis.clone())).unwrap();
    assert!(genesis == blocktree.get_genesis().unwrap());

    let mut tx_builder1 = TransactionBuilder::new();
    tx_builder1.add_transfer(&sk1, &pk1, &pk2, 11, 0);
    tx_builder1.add_transfer(&sk2, &pk2, &pk1, 1, 0);
    let tx1 = tx_builder1.build().unwrap();
    let mut block1 = HashedBlock::new();
    block1.mut_signed_block().mut_block().mut_transactions().push(tx1);
    block1.mut_signed_block().mut_block().set_height(1);
    block1.set_previous_block(&genesis_hash);
    let block1_hash = block1.compute_hash();
    blocktree.insert_block(block1).unwrap();
    blocktree.set_head(&block1_hash).unwrap();
    println!("balance_pk1: {:?}", blocktree.get_balance(&pk1).unwrap());
    println!("balance_pk2: {:?}", blocktree.get_balance(&pk2).unwrap());
    println!("balance_pk3: {:?}", blocktree.get_balance(&pk3).unwrap());
    assert!(970 == blocktree.get_balance(&pk1).unwrap().get_tokens());
    assert!(1 == blocktree.get_balance(&pk1).unwrap().get_op_index());
    assert!(83 == blocktree.get_balance(&pk2).unwrap().get_tokens());
    assert!(1 == blocktree.get_balance(&pk2).unwrap().get_op_index());
    assert!(10 == blocktree.get_balance(&pk3).unwrap().get_tokens());
    assert!(0 == blocktree.get_balance(&pk3).unwrap().get_op_index());

    let mut tx_builder2 = TransactionBuilder::new();
    tx_builder2.add_transfer(&sk2, &pk2, &pk3, 10, 1);
    let tx2 = tx_builder2.build().unwrap();
    let mut block2 = HashedBlock::new();
    block2.mut_signed_block().mut_block().mut_transactions().push(tx2);
    block2.mut_signed_block().mut_block().set_height(2);
    block2.set_previous_block(&block1_hash);
    let block2_hash = block2.compute_hash();
    blocktree.insert_block(block2).unwrap();
    blocktree.set_head(&block2_hash).unwrap();
    assert!(970 == blocktree.get_balance(&pk1).unwrap().get_tokens());
    assert!(1 == blocktree.get_balance(&pk1).unwrap().get_op_index());
    assert!(73 == blocktree.get_balance(&pk2).unwrap().get_tokens());
    assert!(2 == blocktree.get_balance(&pk2).unwrap().get_op_index());
    assert!(20 == blocktree.get_balance(&pk3).unwrap().get_tokens());
    assert!(0 == blocktree.get_balance(&pk3).unwrap().get_op_index());

    let mut tx_builder3 = TransactionBuilder::new();
    tx_builder3.add_transfer(&sk1, &pk1, &pk3, 100, 0);
    tx_builder3.add_transfer(&sk2, &pk2, &pk1, 7, 0);
    let tx3 = tx_builder3.build().unwrap();
    let mut block3 = HashedBlock::new();
    block3.mut_signed_block().mut_block().mut_transactions().push(tx3);
    block3.mut_signed_block().mut_block().set_height(1);
    block3.set_previous_block(&genesis_hash);
    let block3_hash = block3.compute_hash();
    blocktree.insert_block(block3).unwrap();

    {
        let at_genesis = blocktree.snapshot_at(&genesis_hash).unwrap();
        assert!(980 == at_genesis.get_balance(&pk1).unwrap().get_tokens());
        assert!(0  == at_genesis.get_balance(&pk1).unwrap().get_op_index());
        assert!(73 == at_genesis.get_balance(&pk2).unwrap().get_tokens());
        assert!(0 == at_genesis.get_balance(&pk2).unwrap().get_op_index());
        assert!(10 == at_genesis.get_balance(&pk3).unwrap().get_tokens());
        assert!(0 == at_genesis.get_balance(&pk3).unwrap().get_op_index());
    }
    {
        let at_block1 = blocktree.snapshot_at(&block1_hash).unwrap();
        assert!(970 == at_block1.get_balance(&pk1).unwrap().get_tokens());
        assert!(1 == at_block1.get_balance(&pk1).unwrap().get_op_index());
        assert!(83 == at_block1.get_balance(&pk2).unwrap().get_tokens());
        assert!(1 == at_block1.get_balance(&pk2).unwrap().get_op_index());
        assert!(10 == at_block1.get_balance(&pk3).unwrap().get_tokens());
        assert!(0 == at_block1.get_balance(&pk3).unwrap().get_op_index());
    }
    {
        let at_block3 = blocktree.snapshot_at(&block3_hash).unwrap();
        println!("{:?}", at_block3.get_balance(&pk1).unwrap());
        println!("{:?}", at_block3.get_balance(&pk1).unwrap());
        assert!(887 == at_block3.get_balance(&pk1).unwrap().get_tokens());
        assert!(1 == at_block3.get_balance(&pk1).unwrap().get_op_index());
        assert!(66 == at_block3.get_balance(&pk2).unwrap().get_tokens());
        assert!(1 == at_block3.get_balance(&pk2).unwrap().get_op_index());
        assert!(110 == at_block3.get_balance(&pk3).unwrap().get_tokens());
        assert!(0 == at_block3.get_balance(&pk3).unwrap().get_op_index());
    }

    blocktree.set_head(&block3_hash).unwrap();
    assert!(887 == blocktree.get_balance(&pk1).unwrap().get_tokens());
    assert!(1 == blocktree.get_balance(&pk1).unwrap().get_op_index());
    assert!(66 == blocktree.get_balance(&pk2).unwrap().get_tokens());
    assert!(1 == blocktree.get_balance(&pk2).unwrap().get_op_index());
    assert!(110 == blocktree.get_balance(&pk3).unwrap().get_tokens());
    assert!(0 == blocktree.get_balance(&pk3).unwrap().get_op_index());
}
