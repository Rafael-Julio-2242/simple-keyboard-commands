extern crate udev;

use evdev::{enumerate, EventType, InputEvent};
use evdev::{Device, Key};

use std::thread;
use std::time::Duration;
use std::process::Command;

fn main() {

    let status = Command::new("gnome-terminal")
        .status()
        .expect("Falha ao iniciar o terminal no Linux");

    if !status.success() {
        eprintln!("Erro ao abrir o terminal");
    }

    let mut device = Device::open("/dev/input/event4").unwrap();
    // check if the device has an ENTER key
    if device.supported_keys().map_or(false, |keys| keys.contains(Key::KEY_ENTER)) {
        
        hello(&mut device);


    } else {
        println!(":(");
    }


}

fn enumerate_devices() {

    let mut enumerator = enumerate();

    while let Some(device) = enumerator.next() {
      let path = device.0;
      let dev = device.1;
      println!("-------------------------------");
      println!("[DEVICE PATH]: {:?}", path );
      println!("[DEVICE NAME]: {}", dev.name().unwrap() );
      println!("-------------------------------");
    }

}


fn hello(device: &mut Device) {
    thread::sleep(Duration::from_millis(2000) );

    aperta_tecla(device, Key::KEY_H.code());
    aperta_tecla(device, Key::KEY_E.code());
    aperta_tecla(device, Key::KEY_L.code());
    aperta_tecla(device, Key::KEY_L.code());
    aperta_tecla(device, Key::KEY_O.code());



}

fn aperta_tecla(device: &mut Device, key_code: u16) {
    device.send_events(&[ InputEvent::new(EventType::KEY, key_code, 1) ]).unwrap();
    device.send_events(&[ InputEvent::new(EventType::KEY, key_code, 0) ]).unwrap();
}

fn abrir_terminal(device: &mut Device) {

    aperta_tecla(device, Key::KEY_LEFTMETA.code());

    thread::sleep(Duration::from_millis(2000) );
    
    aperta_tecla(device, Key::KEY_T.code());
    aperta_tecla(device, Key::KEY_E.code());
    aperta_tecla(device, Key::KEY_R.code());
    aperta_tecla(device, Key::KEY_M.code());
    aperta_tecla(device, Key::KEY_I.code());
    aperta_tecla(device, Key::KEY_N.code());
    aperta_tecla(device, Key::KEY_A.code());
    aperta_tecla(device, Key::KEY_L.code());

    thread::sleep(Duration::from_millis(2000) );

    deep_press(device, Key::KEY_ENTER.code());
    aperta_tecla(device, Key::KEY_DOWN.code());

    thread::sleep(Duration::from_millis(2000) );


}

fn deep_press(device: &mut Device, key_code: u16) {
    device.send_events(&[ InputEvent::new(EventType::KEY, key_code, 1) ]).unwrap();
}

// fn simulate_key_press(device_path: &str, key_code: c_int) {


//   let device = EvdevDevice::open(device_path);

//     match device {
//         Ok(device) => {
            
//         },
//         Err(err) => println!("Error: {:?}", err),
//     }



// }