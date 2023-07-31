mod interface;

use interface::*;

use prusti_contracts::*;

fn main() {
    let mut obj = pyt_new_object(12, &DoublingType {});
    assert!(obj.data == 12);
    pyt_apply_action(&mut obj);
    assert!(obj.data == 24);
}
