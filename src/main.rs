use bluer::gatt::local::{
    Application, Characteristic, CharacteristicRead, CharacteristicReadRequest,
    CharacteristicWrite, CharacteristicWriteRequest, Service,
};
use bluer::{Adapter, AdapterEvent, Result};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, BufReader};
use uuid::Uuid;

// Custom service UUID - you can change this
const SERVICE_UUID: Uuid = Uuid::from_u128(0x12345678_1234_1234_1234_123456789abc);
const CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x87654321_4321_4321_4321_cba987654321);

#[derive(Debug, Clone)]
struct DeviceCharacteristic {
    value: Vec<u8>,
}

impl DeviceCharacteristic {
    fn new() -> Self {
        Self {
            value: b"Hello from Rust BLE Peripheral!".to_vec(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting Rust BLE Peripheral");

    // Get the default Bluetooth adapter
    let adapter = get_default_adapter().await?;
    
    // Setup GATT application
    let app = create_gatt_application().await?;
    
    // Register GATT application
    let app_handle = adapter.serve_gatt_application(app).await?;
    info!("GATT application registered");

    // Setup advertisement
    setup_advertisement(&adapter).await?;
    
    info!("BLE Peripheral is running. Press 'q' to quit.");
    
    // Handle events and user input
    let stdin = tokio::io::stdin();
    let mut lines = BufReader::new(stdin).lines();
    
    loop {
        tokio::select! {
            // Handle adapter events
            event = adapter.events().await => {
                match event {
                    Ok(AdapterEvent::DeviceAdded(addr)) => {
                        info!("Device added: {}", addr);
                    }
                    Ok(AdapterEvent::DeviceRemoved(addr)) => {
                        info!("Device removed: {}", addr);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        error!("Adapter event error: {}", e);
                    }
                }
            }
            
            // Handle user input
            line = lines.next_line() => {
                match line {
                    Ok(Some(line)) if line.trim() == "q" => {
                        info!("Shutting down...");
                        break;
                    }
                    Ok(Some(_)) => {
                        info!("Press 'q' to quit");
                    }
                    Ok(None) => break,
                    Err(e) => {
                        error!("Error reading input: {}", e);
                        break;
                    }
                }
            }
        }
    }

    // Cleanup
    drop(app_handle);
    info!("BLE Peripheral stopped");
    Ok(())
}

async fn get_default_adapter() -> Result<Adapter> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    
    info!("Using adapter: {}", adapter.name());
    info!("Address: {}", adapter.address().await?);
    
    // Ensure adapter is powered on
    adapter.set_powered(true).await?;
    info!("Adapter powered on");
    
    Ok(adapter)
}

async fn create_gatt_application() -> Result<Application> {
    info!("Creating GATT application with service UUID: {}", SERVICE_UUID);
    
    let characteristic = DeviceCharacteristic::new();
    
    let app = Application {
        services: vec![Service {
            uuid: SERVICE_UUID,
            primary: true,
            characteristics: vec![Characteristic {
                uuid: CHARACTERISTIC_UUID,
                read: Some(CharacteristicRead {
                    read: true,
                    fun: Box::new(move |req| {
                        let char_clone = characteristic.clone();
                        Box::pin(async move {
                            debug!("Read request from device: {:?}", req.device_path);
                            Ok(char_clone.value)
                        })
                    }),
                    ..Default::default()
                }),
                write: Some(CharacteristicWrite {
                    write: true,
                    write_without_response: true,
                    fun: Box::new(move |req| {
                        Box::pin(async move {
                            info!("Write request: {:?}", String::from_utf8_lossy(&req.value));
                            info!("From device: {:?}", req.device_path);
                            Ok(())
                        })
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    
    Ok(app)
}

async fn setup_advertisement(adapter: &Adapter) -> Result<()> {
    info!("Setting up advertisement");
    
    let le_advertisement = bluer::adv::Advertisement {
        advertisement_type: bluer::adv::Type::Peripheral,
        service_uuids: vec![SERVICE_UUID].into_iter().collect(),
        local_name: Some("RustBLE".to_string()),
        discoverable: Some(true),
        tx_power: Some(0),
        ..Default::default()
    };

    let _adv_handle = adapter.advertise(le_advertisement).await?;
    info!("Advertisement started - device should now be discoverable as 'RustBLE'");
    
    Ok(())
}