#[test] fn layout() -> Result<(), Box<dyn std::error::Error>> {
    use crate::*;

    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use core::mem::*;

    let mut code_path = std::env::current_exe().expect("current_exe()");
    code_path.pop();
    let out_dir = code_path.clone();

    code_path.push("interop-layout-test.cpp");
    let code_path : &Path = &code_path;

    let bin_path  = "interop-layout-test.o";
    let mut code = std::io::BufWriter::new(File::create(code_path).expect("couldn't create code_path"));

    writeln!(code, "#define WIN32_LEAN_AND_MEAN")?;
    writeln!(code, "#include <windows.h>")?;
    writeln!(code, "#include <xinput.h>")?; // XINPUT_*
    writeln!(code, "#include <cstddef>")?; // offsetof

    macro_rules! structs {
        ($(
            $rust_struct:ident => $cpp_struct:ident {
                $( $rust_field:ident => $cpp_field:ident ),* $(,)?
            }
        )*) => {{$(
            let rust_align = align_of::<$rust_struct>();
            let rust_size  =  size_of::<$rust_struct>();
            let rust_struct_name  = stringify!($rust_struct);
            let cpp_struct_name   = stringify!($cpp_struct);

            writeln!(code)?;
            writeln!(code)?;
            writeln!(code)?;
            writeln!(code, "static_assert({rust_align: >2} == alignof({cpp_struct_name}), \"alignof(xinput::{rust_struct_name}) != alignof({cpp_struct_name})\");")?;
            writeln!(code, "static_assert({rust_size: >2} ==  sizeof({cpp_struct_name}),  \"sizeof(xinput::{rust_struct_name}) != sizeof({cpp_struct_name})\");")?;

            let _rust_struct = <$rust_struct as bytemuck::Zeroable>::zeroed();
            $({
                let rust_align      = align_of_val(&_rust_struct.$rust_field);
                let rust_size       =  size_of_val(&_rust_struct.$rust_field);
                let rust_offset     = { let base : *const _ = &_rust_struct; let field : *const _ = &_rust_struct.$rust_field; (field as usize) - (base as usize) };
                let rust_field_name = stringify!($rust_field);
                let cpp_field_name  = stringify!($cpp_field);

                writeln!(code)?;
                writeln!(code, "static_assert({rust_align: >2} == alignof(decltype({cpp_struct_name}{{}}.{cpp_field_name})), \"alignof(xinput::{rust_struct_name}::{rust_field_name}) != alignof({cpp_struct_name}::{cpp_field_name})\");")?;
                writeln!(code, "static_assert({rust_size: >2} ==  sizeof(decltype({cpp_struct_name}{{}}.{cpp_field_name})),  \"sizeof(xinput::{rust_struct_name}::{rust_field_name}) != sizeof({cpp_struct_name}::{cpp_field_name})\");")?;
                writeln!(code, "static_assert({rust_offset: >2} == offsetof({cpp_struct_name}, {cpp_field_name}), \"offsetof(xinput::{rust_struct_name}, {rust_field_name}) != offsetof({cpp_struct_name}, {cpp_field_name})\");")?;
            })*
        )*}};
    }

    structs! {
        BatteryInformation => XINPUT_BATTERY_INFORMATION {
            battery_type        => BatteryType,
            battery_level       => BatteryLevel,
        }

        Capabilities => XINPUT_CAPABILITIES {
            ty                  => Type,
            sub_type            => SubType,
            flags               => Flags,
            gamepad             => Gamepad,
            vibration           => Vibration,
        }

        Gamepad => XINPUT_GAMEPAD {
            buttons             => wButtons,
            left_trigger        => bLeftTrigger,
            right_trigger       => bRightTrigger,
            left_thumb_x        => sThumbLX,
            left_thumb_y        => sThumbLY,
            right_thumb_x       => sThumbRX,
            right_thumb_y       => sThumbRY,
        }

        DSoundAudioDeviceGuid => GUID {
            // fields not individually defined
        }

        Keystroke => XINPUT_KEYSTROKE {
            virtual_key         => VirtualKey,
            unicode             => Unicode,
            flags               => Flags,
            user_index          => UserIndex,
            hid_code            => HidCode,
        }

        State => XINPUT_STATE {
            packet_number       => dwPacketNumber,
            gamepad             => Gamepad,
        }

        Vibration => XINPUT_VIBRATION {
            left_motor_speed    => wLeftMotorSpeed,
            right_motor_speed   => wRightMotorSpeed,
        }
    }

    drop(code);

    for target in ["x86_64-pc-windows-msvc", "i686-pc-windows-msvc"] {
        cc::Build::new()
            .file(code_path)
            .out_dir(&out_dir)
            .opt_level(0)
            .target(target)
            .host(target)
            .cpp(true)
            .emit_rerun_if_env_changed(false)
            //.std("c++11") // alignof, static_assert
            .try_compile(bin_path)?;
    }

    Ok(())
}
