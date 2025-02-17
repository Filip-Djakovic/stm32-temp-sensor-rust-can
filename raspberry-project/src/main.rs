use socketcan::{frame, CanFrame, CanSocket, EmbeddedFrame, Socket, StandardId};
static mut temp: f32 = 0.0;
fn main() {
    println!("RUN");

        recive_message();
    
    
    //send_message();
}

fn recive_message(){ // function for receiving message from stm32
    let socket = CanSocket::open("can0").expect("Failed with open can0 in recive method");
    println!("Before receiving the message");
    match socket.read_frame() {
        Ok(frame) => {
            let id = frame.id();
            let data = frame.data(); 
            let temp_c:f32 = f32::from_le_bytes(data.try_into().expect("Error with convert number"));
            if (temp_c > 25.0){
                println!("Message to turn on led diode");
                send_message(10);
            }
            else {
                send_message(5);
                println!("Message to turn off led diode");
            }
            println!("Temperature is {}", temp_c);
        }
        Err(err) =>{
            println!("Message error {}",err);
        }
    }
    /*let frame = socket.read_frame().expect("Failed with recived message");
    println!("Poslije primanja poruke");
    let data = frame.data();
    let id = frame.id();
    let tempC = f32::from_le_bytes(data.try_into().expect("Error with convert number"));
    println!("Recived temp is {}", tempC);*/
 }

fn send_message(number: u32){ //function for sending message to stm32
    let socket = CanSocket::open("can0").expect("Failed with opetn can0 in send method");
    //let  number:u32 = 10;
    /*if (unsafe { temp } > 25.0){
        number = 1;
    }*/
    let data = number.to_le_bytes();
    let id = StandardId::new(0x123).expect("Failed with create ID");
    let frame = CanFrame::new(id, &data).expect("Failed with create frame");
    socket.write_frame(&frame).expect("Failed with send frame");
}
