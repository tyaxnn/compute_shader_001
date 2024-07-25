pub mod about_status{

    #[derive(Debug, Copy, Clone)]
    pub enum PinPongStatus{
        FtT2,
        F1T2,
        F2T1,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Status{
        pub buffer_size : u64,
        pub frame_len : u32,
        pub frame_rate : u32,
        pub elapsed_frame : u32,
        pub next_frame_index : u32,
        pub start_time : std::time::Instant,
        pub ping_pong : PinPongStatus,
    }

    impl Status {
        pub fn new() -> Status{

            let buffer_size = 20;
            let frame_len = 200;
            let frame_rate = 60;
            let elapsed_frame = 0;
            let next_frame_index = 0; 
            let start_time = std::time::Instant::now();
            let ping_pong = PinPongStatus::FtT2;

            Status{
                buffer_size,
                frame_len,
                frame_rate,
                elapsed_frame,
                next_frame_index,
                start_time,
                ping_pong,
            }
        } 
    }
}