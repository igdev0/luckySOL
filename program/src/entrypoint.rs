use solana_program::entrypoint;

use crate::processor::processor;

entrypoint!(processor);
