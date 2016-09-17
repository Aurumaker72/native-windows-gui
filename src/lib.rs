/*!
    A very high level native gui library for Windows.
*/

extern crate winapi;

pub mod controls;

use std::hash::Hash;
use std::collections::HashMap;
use winapi::HWND;
use controls::ControlTemplate;

type ControlCollection<ID> = HashMap<ID, HWND>;

/**
    A single threaded window manager.
*/
pub struct Ui<ID: Eq+Clone+Hash > {
    controls: *mut ControlCollection<ID>
}

impl<ID: Eq+Clone+Hash> Ui<ID> {

    /**
        Creates a new `Ui` that will manage the interface on the 
        current thread.
    */
    pub fn new() -> Ui<ID> {
        let controls = ControlCollection::<ID>::new();
        let controls_raw = Box::into_raw(Box::new(controls));
        Ui{controls: controls_raw}
    }

    /**
        Create a new control in the ui manager from the provided template
        and associate it with the ID provided by the user.

        If the control creation succeeded, return the id used by the user. 

        If the control creation somehow failed return `Err`
    */
    pub fn new_control<T:ControlTemplate<ID>>(&mut self, cont: ID, template: T) -> Result<ID, ()> {
        let controls: &mut ControlCollection<ID> = unsafe{ &mut *self.controls };
        
        if !controls.contains_key(&cont) {
            let handle: HWND;
            match template.create(self, cont.clone()) {
                Ok(h) => handle = h,
                Err(_) => { 
                    return Err(());  // Error: Template creation failed: *template error* 
                }
            }

            controls.insert(cont.clone(), handle);
            Ok(cont)
        } else {
            Err(()) // Error: A widget with this id already exists
        }

    }

}
