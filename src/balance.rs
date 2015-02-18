use std::cell::RefCell;
use std::collections::hash_map::{self, HashMap};
use std::fmt;

use rustc_serialize::base64::{self, ToBase64};

use crypto::PublicKey;
use block::HashedBlockExt;
use error::{SimplesError, SimplesResult};
use simples_pb::{Balance, BalancePatch, BlockPatch, HashedBlock,
                 Transaction, Transfer};

pub fn make_genesis_patch(genesis: &HashedBlock) -> SimplesResult<BlockPatch> {
    try!(genesis.verify());
    let txes = genesis.get_block().get_transactions();
    match txes.len() {
        0 => Ok(BlockPatch::new()),
        1 => {
            let mut cache = HashMap::<PublicKey, u64>::new();
            let commit = txes[0].get_commit();
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
            let mut block_patch = BlockPatch::new();
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
                block_patch.mut_patches().push(patch);
            }
            Ok(block_patch)
        },
        _ => Err(SimplesError::new(
            "Genesis block must contain at most 1 transaction."))
    }
}

pub trait BalancePatchExt {
    fn decode_public_key(&self) -> SimplesResult<PublicKey>;
    fn reverse(&self) -> Self;
}

impl BalancePatchExt for BalancePatch {
    fn decode_public_key(&self) -> SimplesResult<PublicKey> {
        PublicKey::from_bytes(&self.get_public_key())
    }

    fn reverse(&self) -> Self {
        let mut reversed = BalancePatch::new();
        reversed.set_public_key(self.get_public_key().to_vec());
        reversed.set_before(self.get_after().clone());
        reversed.set_after(self.get_before().clone());
        reversed
    }
}

pub trait LedgerReader {
    fn get_balance(&self, address: &PublicKey) -> SimplesResult<Balance>;
}

pub trait LedgerWriter {
    fn set_balance(&mut self, address: &PublicKey, balance: Balance)
                   -> SimplesResult<()>;
}

pub trait Patchable {
    fn apply_patch(&mut self, patch: BalancePatch) -> SimplesResult<()>;
}

impl<LedgerMutator: LedgerReader + LedgerWriter>
    Patchable for LedgerMutator {
    fn apply_patch(&mut self, patch: BalancePatch) -> SimplesResult<()> {
        let public_key = try!(PublicKey::from_bytes(patch.get_public_key()));
        let before_balance = try!(self.get_balance(&public_key));
        if before_balance != *patch.get_before() {
            return Err(SimplesError::new(&format!(
                "Actual initial balance ({}, {}) != ({}, {}) from the patch.",
                before_balance.get_tokens(), before_balance.get_op_index(),
                patch.get_before().get_tokens(),
                patch.get_before().get_op_index())[]));
        }
        try!(self.set_balance(&public_key, patch.get_after().clone()));
        Ok(())
    }
}

pub struct LedgerSnapshot<'a, LedgerReadOnly: 'a + LedgerReader> {
    cache: RefCell<HashMap<PublicKey, BalancePatch>>,
    store: &'a LedgerReadOnly
}

impl<'a, L: 'a + LedgerReader> LedgerSnapshot<'a, L> {
    pub fn new(store: &'a L) -> LedgerSnapshot<'a, L> {
        LedgerSnapshot {
            cache: RefCell::new(HashMap::<PublicKey, BalancePatch>::new()),
            store: store
        }
    }

    pub fn clear(&mut self) { self.cache.borrow_mut().clear(); }

    pub fn add_transfer(&mut self, transfer: &Transfer) ->
        SimplesResult<()>
    {
        let source_pk = try!(PublicKey::from_bytes(transfer.get_source_pk()));
        let destination_pk =
            try!(PublicKey::from_bytes(transfer.get_destination_pk()));
        let mut source = try!(self.get_balance(&source_pk));
        let mut destination = try!(self.get_balance(&destination_pk));

        if source.get_tokens() >= transfer.get_tokens() {
            // println!("source: {}",
            //          transfer.get_source_pk().to_base64(base64::STANDARD));
            // println!("destination: {}",
            //          transfer.get_destination_pk().to_base64(base64::STANDARD));
            if transfer.get_op_index() == source.get_op_index() {
                // println!("num: {} / tokens: {} / balance: {} -> {}\n",
                //          source.get_op_index(), transfer.get_tokens(),
                //          source.get_tokens(),
                //          source.get_tokens() - transfer.get_tokens());

                let source_tokens = source.get_tokens() - transfer.get_tokens();
                let source_op_index = source.get_op_index() + 1;
                let dest_tokens = destination.get_tokens() + transfer.get_tokens();
                source.set_tokens(source_tokens);
                source.set_op_index(source_op_index);
                destination.set_tokens(dest_tokens);

                try!(self.set_balance(&source_pk, source));
                try!(self.set_balance(&destination_pk, destination));
                Ok(())
            } else {
                Err(SimplesError::new(&format!(
                    "Wrong op number for source address {}: op_index was
{} != {} (required)", source_pk, transfer.get_op_index(), source.get_op_index())[]))
            }
        } else {
            Err(SimplesError::new(&format!(
                "ERROR: Not enough funds. Source address balance is {} but
{} tokens were transferred: {} -> {}", source.get_tokens(), transfer.get_tokens(),
                source_pk, destination_pk)[]))
        }
    }

    pub fn apply_transaction(
        &mut self, transaction: &Transaction) -> SimplesResult<()> {
        for transfer in transaction.get_commit().get_transfers().iter() {
            try!(self.add_transfer(transfer));
        }
        Ok(())
    }

    pub fn apply_block(&mut self, block: &HashedBlock) -> SimplesResult<()> {
        for transaction in block.get_block().get_transactions().iter() {
            try!(self.apply_transaction(transaction));
        }
        Ok(())
    }

    pub fn make_patches(&self) -> Vec<BalancePatch> {
        let mut block_patch = Vec::<BalancePatch>::new();
        for patch in self.cache.borrow().values() {
            block_patch.push(patch.clone());
        }
        block_patch
    }
}

impl<'a, L: 'a + LedgerReader> fmt::Debug for LedgerSnapshot<'a, L> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.cache.borrow())
    }
}

impl<'a, L: 'a + LedgerReader> LedgerReader for LedgerSnapshot<'a, L> {
    fn get_balance(&self, address: &PublicKey) -> SimplesResult<Balance> {
        match self.cache.borrow_mut().entry(address.clone()) {
            hash_map::Entry::Occupied(occupied_patch) =>
                Ok(occupied_patch.get().get_after().clone()),
            hash_map::Entry::Vacant(vacant_patch) => {
                let db_balance = try!(self.store.get_balance(address));
                let mut patch = BalancePatch::new();
                patch.set_public_key(address.0.to_vec());
                patch.set_before(db_balance.clone());
                patch.set_after(db_balance.clone());
                vacant_patch.insert(patch);
                Ok(db_balance)
            }
        }
    }
}

impl<'a, LedgerReadOnly: 'a + LedgerReader> LedgerWriter
    for LedgerSnapshot<'a, LedgerReadOnly> {
    fn set_balance(&mut self, address: &PublicKey, balance: Balance)
                   -> SimplesResult<()> {
        match self.cache.borrow_mut().entry(address.clone()) {
            hash_map::Entry::Occupied(mut occupied_patch) => {
                occupied_patch.get_mut().set_after(balance);
            },
            hash_map::Entry::Vacant(vacant_patch) => {
                let db_balance = try!(self.store.get_balance(address));
                let mut patch = BalancePatch::new();
                patch.set_public_key(address.0.to_vec());
                patch.set_before(db_balance);
                patch.set_after(balance);
                vacant_patch.insert(patch);
            }
        };
        Ok(())
    }
}

/*****  Tests  *****/

#[test]
fn test_make_genesis_patch_empty() {
    use block::GenesisBuilder;

    let builder = GenesisBuilder::new();
    let genesis = builder.build();
    let maybe_patch = make_genesis_patch(&genesis);
    assert!(maybe_patch.is_ok());
    assert!(0 == maybe_patch.unwrap().get_patches().len());
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
    let maybe_patch = make_genesis_patch(&genesis);
    assert!(maybe_patch.is_ok());
    assert!(2 == maybe_patch.as_ref().unwrap().get_patches().len());
    let patch_map: HashMap<PublicKey, BalancePatch> =
        FromIterator::from_iter(maybe_patch.unwrap().get_patches().iter().map(
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

struct TestLedgerOneAddress {
    address: PublicKey,
    balance: Balance
}

impl LedgerReader for TestLedgerOneAddress {
    fn get_balance(&self, address: &PublicKey) -> SimplesResult<Balance> {
        if self.address == *address {
            Ok(self.balance.clone())
        } else {
            Err(SimplesError::new(""))
        }
    }
}

impl LedgerWriter for TestLedgerOneAddress {
    fn set_balance(&mut self, address: &PublicKey, balance: Balance)
                   -> SimplesResult<()> {
        if self.address == *address {
            self.balance = balance;
            Ok(())
        } else {
            Err(SimplesError::new(""))
        }
    }
}

struct TestLedgerHashMap {
    ledger: RefCell<HashMap<PublicKey, Balance>>
}

impl LedgerReader for TestLedgerHashMap {
    fn get_balance(&self, address: &PublicKey) -> SimplesResult<Balance> {
        match self.ledger.borrow_mut().entry(address.clone()) {
            hash_map::Entry::Occupied(occupied) => Ok(occupied.get().clone()),
            hash_map::Entry::Vacant(vacant) => {
                let mut balance = Balance::new();
                balance.set_tokens(0);
                balance.set_op_index(0);
                vacant.insert(balance.clone());
                Ok(balance)
            }
        }
    }
}

#[test]
fn test_patchable_apply_patch() {
    use crypto::gen_keypair;

    let (pk1, sk1) = gen_keypair();
    let (pk2, sk2) = gen_keypair();
    let mut initial_balance = Balance::new();
    initial_balance.set_tokens(1000);
    initial_balance.set_op_index(7);
    let mut mutator = TestLedgerOneAddress {
        address: pk2.clone(),
        balance: initial_balance.clone()
    };

    let mut patch = BalancePatch::new();
    assert!(mutator.apply_patch(patch.clone()).is_err());
    assert!(initial_balance == mutator.balance);

    patch.set_public_key(pk1.0.to_vec());
    patch.mut_before().set_tokens(999);
    patch.mut_before().set_tokens(7);
    assert!(mutator.apply_patch(patch.clone()).is_err());
    assert!(initial_balance == mutator.balance);

    patch.mut_before().set_tokens(1000);
    patch.mut_before().set_op_index(7);
    patch.mut_after().set_tokens(499);
    patch.mut_after().set_op_index(17);
    assert!(mutator.apply_patch(patch.clone()).is_err());

    patch.set_public_key(pk2.0.to_vec());
    assert!(mutator.apply_patch(patch.clone()).is_ok());
    assert!(499 == mutator.balance.get_tokens());
    assert!(17 == mutator.balance.get_op_index());

    patch = patch.reverse();
    assert!(mutator.apply_patch(patch.clone()).is_ok());
    assert!(1000 == mutator.balance.get_tokens());
    assert!(7 == mutator.balance.get_op_index());
}

#[test]
fn test_ledger_snapshot_make_patches() {
    use crypto::gen_keypair;

    let (pk1, sk1) = gen_keypair();
    let (pk2, sk2) = gen_keypair();
    let (pk3, sk3) = gen_keypair();

    let mut balance_pk1 = Balance::new();
    balance_pk1.set_tokens(1000);
    balance_pk1.set_op_index(7);
    let mut ledger_cell = RefCell::new(HashMap::new());
    ledger_cell.borrow_mut().insert(pk1.clone(), balance_pk1.clone());
    let mut ledger = TestLedgerHashMap { ledger: ledger_cell };

    let mut snapshot = LedgerSnapshot::new(&ledger);
    assert!(0 == snapshot.make_patches().len());
    balance_pk1.set_tokens(400);
    balance_pk1.set_op_index(91);
    snapshot.set_balance(&pk1, balance_pk1.clone());
    println!("{:?}", snapshot.make_patches());
}
