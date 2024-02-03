use crate::*;

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, FromInto })]
    BatteryInformation => XINPUT_BATTERY_INFORMATION {
        battery_type    => BatteryType,
        battery_level   => BatteryLevel,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Capabilities => XINPUT_CAPABILITIES {
        ty          => Type,
        sub_type    => SubType,
        flags       => Flags,
        gamepad     => Gamepad,
        vibration   => Vibration,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Gamepad => XINPUT_GAMEPAD {
        buttons         => wButtons,
        left_trigger    => bLeftTrigger,
        right_trigger   => bRightTrigger,
        #[renamed] left_thumb_x    => sThumbLX,
        #[renamed] left_thumb_y    => sThumbLY,
        #[renamed] right_thumb_x   => sThumbRX,
        #[renamed] right_thumb_y   => sThumbRY,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    #[renamed] DSoundAudioDeviceGuid => GUID {
        // fields not individually defined
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Keystroke => XINPUT_KEYSTROKE {
        virtual_key     => VirtualKey,
        unicode         => Unicode,
        flags           => Flags,
        user_index      => UserIndex,
        hid_code        => HidCode,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    State => XINPUT_STATE {
        packet_number   => dwPacketNumber,
        gamepad         => Gamepad,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Vibration => XINPUT_VIBRATION {
        left_motor_speed    => wLeftMotorSpeed,
        right_motor_speed   => wRightMotorSpeed,
    }
}
