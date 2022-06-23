use crate::{Config};
use sp_core::H256;
use frame_support::{parameter_types, weights::Weight};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};
use frame_system as system;
use crate as pallet_cere_ddc;

// Configure a mock runtime to test the pallet.

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>; 
type Block = frame_system::mocking::MockBlock<Test>;  

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block, 
		NodeBlock = Block, 
		UncheckedExtrinsic = UncheckedExtrinsic, 
	{
		System: system::{Pallet, Call, Config, Storage, Event<T>},
		CereDDCModule: pallet_cere_ddc::{Pallet, Call, Storage, Event<T>},
	}
);

impl system::Config for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type PalletInfo = PalletInfo;
	type SS58Prefix = ();
	type SystemWeightInfo = ();
	type OnSetCode = ();
}

parameter_types! {
	pub const MinLength: usize = 8;
	pub const MaxLength: usize = 10;
}

impl Config for Test {
	type Event = ();
	type MinLength = MinLength;
	type MaxLength = MaxLength;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
