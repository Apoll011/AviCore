# Avi MQTT Topics & Context Structure

## Overview

Avi uses **MQTT pub/sub messaging** for real-time communication between the Core Node and all connected devices. This document explains the topic structure and the shared context object that maintains system state.

---

## MQTT Topic Structure

### Topic Hierarchy

```
global
intent
  ├── execute/text
  └── reply/cancel
speak
  └── {peerId}/text
listening
  └── {peerId}/start
```

### Topic Descriptions

#### `global`
- **Direction**: Bidirectional (Core ↔ Devices)
- **Purpose**: System-wide announcements and broadcasts
- **Examples**:
  - Device join/leave notifications
  - System shutdown warnings
  - Network configuration changes
  - Core Node status updates

#### `intent/execute/text`
- **Direction**: Device → Core
- **Purpose**: Submit text-based intent requests for processing
- **Usage**: When a device (like a microphone or UI) has recognized text input and wants the Core to parse the intent and execute actions
- **Example Payload**:
  ```json
  {
    "text": "turn on the kitchen lights",
    "device_id": "mic-livingroom-1",
    "trace_id": "uuid"
  }
  ```

#### `intent/reply/cancel`
- **Direction**: Core → Device
- **Purpose**: Cancel an in-progress reply
- **Usage**: Stop listening to a reply and return to intent mode

#### `speak/{peerId}/text`
- **Direction**: Core → Device
- **Purpose**: Send text-to-speech output to a specific speaker device
- **Dynamic Topic**: `{peerId}` is replaced with the target device's ID
- **Examples**:
  - `speak/speaker-kitchen-1/text`
  - `speak/mic-livingroom-1/text` (if device has speaker capability)
- **Example Payload**:
  ```json
  {
    "text": "The kitchen lights are now on",
    "priority": "normal"
  }
  ```

#### `listening/{peerId}/start`
- **Direction**: Core → Device
- **Purpose**: Signal a specific device to start listening for voice input
- **Dynamic Topic**: `{peerId}` is replaced with the target microphone device's ID
- **Usage**: Core tells a specific mic to activate and start capturing audio
- **Examples**:
  - `listening/mic-bedroom-1/start`
  - `listening/wearable-pin-1/start`

---

## Context Structure

The **Context** is a JSON-like object that maintains the current state of the Avi system. It's shared across all messages and provides devices with awareness of the overall system state.

### Context Schema

```
avi
  ├── core: {peerId}
  ├── device
  │     └── caps: *
  │           └── {peerId}: DeviceCapabilities
  └── dialogue
        ├── speaker: {peerId}
        └── listener: {peerId}
```

### Context Breakdown

#### `avi.core`
- **Type**: String (device peer ID)
- **Purpose**: Identifies which device is acting as the Core Node
- **Example**: `"core-main-uuid"`
- **Usage**: Devices know where to route messages and who is orchestrating the system

#### `avi.device.caps.*`
- **Type**: Map of device IDs to capability objects
- **Purpose**: Stores the full capability declaration for every registered device
- **Structure**: Each key is a `{peerId}` (device ID), and the value is a `DeviceCapabilities` object
- **Example**:
  ```json
  {
    "mic-livingroom-1": {
      "compute": {
        "cpu": { "cores": 4, "architecture": "arm64", ... },
        "ram": { "total_mb": 2048, "free_mb": 1024, ... }
      },
      "sensors": {
        "microphone": {
          "present": true,
          "array_size": 2,
          "sampling_rate_khz": 16,
          "max_spl_db": 110
        }
      },
      "audio": {
        "output": {
          "present": true,
          "channels": 2,
          "max_spl_db": 85
        }
      }
    }
  }
  ```

##### Device Capability Categories

A `DeviceCapabilities` object contains the following top-level categories:

**compute**
- CPU, GPU, NPU, RAM, and storage information
- Used for determining which device can handle local processing tasks

**sensors**
- Microphone, Camera, Temperature, IMU, Lidar, Proximity, Ambient Light, Pressure, Humidity
- Each sensor type includes its specifications and current readings

**connectivity**
- WiFi, Bluetooth, Ethernet, Cellular, UWB, Zigbee, Thread
- Includes signal strength, standards supported, and network status

**power**
- Power source (battery, wired, solar, hybrid)
- Battery percentage, charging status, power draw

**health**
- Uptime, thermal headroom, CPU load, memory pressure, network stability
- Real-time health metrics for system monitoring

**display**
- Screen resolution, refresh rate, touch capability, brightness

**audio**
- Speaker output specifications, spatial audio support, supported formats

**extended**
- Custom capabilities specific to specialized devices
- Flexible key-value structure for future expansion

#### `avi.dialogue.speaker`
- **Type**: String (device peer ID)
- **Purpose**: Identifies which device should output audio responses in the current conversation
- **Example**: `"speaker-kitchen-1"`
- **Usage**: When the Core needs to speak a response, it publishes to `speak/{speaker}/text`

#### `avi.dialogue.listener`
- **Type**: String (device peer ID)
- **Purpose**: Identifies which device should capture the next voice input
- **Example**: `"mic-livingroom-1"`
- **Usage**: Maintains conversation continuity and context awareness

---

## How Topics and Context Work Together

### Example Flow: Voice Command

1. **User speaks**: "Turn on the kitchen lights"

2. **Listener device publishes**:
   ```
   Topic: intent/execute/text
   Payload: {
     "text": "turn on the kitchen lights",
     "device_id": "mic-livingroom-1",
     "context": {
       "avi": {
         "dialogue": {
           "speaker": "mic-livingroom-1",
           "listener": "mic-livingroom-1"
         }
       }
     }
   }
   ```

3. **Core Node processes** the intent and determines action

4. **Core Node sends command** to light (via AECP protocol, not MQTT)

5. **Core Node responds to user**:
   ```
   Topic: speak/mic-livingroom-1/text
   Payload: {
     "text": "The kitchen lights are now on"
   }
   ```

### Example Flow: Multi-Device Interaction

1. **User presses button** on remote control in bedroom

2. **Button device publishes**:
   ```
   Topic: intent/execute/text
   Payload: {
     "text": "play music",
     "device_id": "remote-bedroom-1",
     "context": {
       "avi": {
         "dialogue": {
           "speaker": "speaker-livingroom-1",  // Core selected best speaker
           "listener": "remote-bedroom-1"
         }
       }
     }
   }
   ```

3. **Core Node publishes response**:
   ```
   Topic: speak/speaker-livingroom-1/text
   Payload: {
     "text": "Playing your favorite playlist"
   }
   ```

Notice how the **speaker** and **listener** are different devices - the context tracks this relationship.

---

## Context Synchronization

### When Context Updates

The context object is updated when:
- A device joins or leaves the network (updates `device.caps`)
- Device capabilities change (hardware updates, sensor readings)
- A new dialogue begins (updates `dialogue.speaker` and `dialogue.listener`)
- The Core Node changes (updates `avi.core`)

### How Devices Access Context

Devices can:
1. **Query their own capabilities**: Look up `device.caps[{self.id}]` to understand their own features
2. **Discover peer capabilities**: Check `device.caps[{other_device_id}]` to see what other devices can do
3. **Understand dialogue flow**: Read `dialogue.speaker` and `dialogue.listener` to know conversation state
4. **Route to Core**: Use `avi.core` to know where to send system-level requests

### Context Persistence

- Context is **in-memory** on the Core Node
- Capability declarations persist across device reconnections
- Dialogue context is **session-scoped** and resets after each interaction completes
- Full context can be reconstructed from device registrations on Core restart