/* POST MVP TODO
 * - Configuration
 *
*/

use std::process::exit;

use pluralkit_rich_presence::run;

fn main() {
	match run() {
		Ok(_) => (),
		Err(e) => {
			println!("Error! {e}");
			exit(1); // Probably not best practice but we don't care :3
		}
	}
}
