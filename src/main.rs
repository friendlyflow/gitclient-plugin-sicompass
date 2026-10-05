//! The program sicompass starts: the git client, served over stdin and stdout.

sicompass_sdk::plugin::main!(gitclient_plugin::GitClientProvider);
