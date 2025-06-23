mod two;
mod boradv;


static  BYTES:[u8; 3] = [1, 2, 3];
static mut BYTES_MUT: [u8; 3] = [1, 2, 3];

fn main() {
    two::out();

    //MUT_BYTES_MUT[0] = 4;

    unsafe {
        BYTES_MUT[0] = 4;
        assert_eq!(BYTES_MUT[0], 4);
    }



}
