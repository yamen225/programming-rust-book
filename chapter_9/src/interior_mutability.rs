// use std::os::fd;

// pub struct SpiderRobot {
//     species: String,
//     web_enabled: bool,
//     leg_device: [fd::FileDesc; 8], // immutable, we cannot write on it.
// }

// use std::rc::Rc;


// pub struct SpiderSenses{
//     robot: Rc<SpiderRobot>, // pointer to settings and I/O
//     eyes: [Camera; 32],
//     motion: Accelerometer,
//     ...
// }

use std::cell::Cell;

pub struct SpiderRobot {
    // ... pider proeprties 
    hardware_error_count: Cell<u32>,
    // ... rest of spider robot properties
}

// we can now update and change the hardware error count

impl SpiderRobot {
    /// increase the error count by 1
    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1 );
    }

    /// True if any hardware errors have been reported.
    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn test_usage_ref_cell() {
        let ref_cell: RefCell<String> = RefCell::new("hello".to_string());
        
        let r = ref_cell.borrow(); // ok, returns a Ref<String>

        let count = r.len(); // Ok, returns "Hello".len()

        assert_eq!(count, 5);

        // let mut w = ref_cell.borrow_mut(); // panic: already borrowed
        // w.push_str(" world"); 
        let ref_cell1: RefCell<String> = RefCell::new("hello".to_string());
        {
            let r1 = ref_cell1.borrow(); // ok, returns a Ref<String>
        }
        let mut w = ref_cell1.borrow_mut(); // will work as the r1 is inside a context
        w.push_str(" world"); 
        let count = w.len();
        assert_eq!(count, "hello world".len());

    }
}

