// Hound -- A wav encoding and decoding library in Rust
// Copyright (C) 2015 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This example shows how to play a wav file using the cpal crate.

extern crate hound;
extern crate cpal;

use std::env;
use std::thread;

fn main() {
    // Make a WavReader that reads the file provided as program argument.
    let file_name = env::args().nth(1).expect("no file given");
    let mut reader = hound::WavReader::open(file_name).unwrap();
    let spec = reader.spec();

    let endpoint = cpal::get_default_endpoint().unwrap();

    // Pick a playback format supported by the endpoint, which matches the spec
    // of the wav file.
    let format = endpoint
        .supported_formats()
        .unwrap()
        .next()
        .expect("no supported playback format")
        .with_max_samples_rate();

    let samples_rate = format.samples_rate.0 as f32;
    let mut sample_clock = 0f32;

    // TODO:
    // Checkout API for WavReader.
    // Grab all samples - check if there is an iterator available.
    // make sure the event loop grabs this next() value.
    let samples = reader.samples().map(|s| s.unwrap()).collect();

    let mut next_value = || {
        samples.next::<f32>();
        // sample_clock = (sample_clock + 1.0) % samples_rate;
        // (sample_clock * 440.0 / samples_rate)
    };


    // Start Build Voice

    let event_loop = cpal::EventLoop::new();
    let mut voice_id = event_loop.build_voice(&endpoint, &format).unwrap();

    event_loop.play(voice_id);


    // End Build Voice
    //
    // Start P.ayback
    event_loop.run(move |_, buffer| {
        match buffer {
            cpal::UnknownTypeBuffer::U16(mut buffer) => {
                for sample in buffer.chunks_mut(format.channels.len()) {
                    // let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    let value = next_value();
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },

            cpal::UnknownTypeBuffer::I16(mut buffer) => {
                for sample in buffer.chunks_mut(format.channels.len()) {
                    // let value = (next_value() * std::i16::MAX as f32) as i16;
                    let value = next_value();
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },

            cpal::UnknownTypeBuffer::F32(mut buffer) => {
                for sample in buffer.chunks_mut(format.channels.len()) {
                    let value = next_value();
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
        };
    });

    // End Playback

    // A voice in cpal is used for playback.
    // let mut voice = cpal::Voice::new(&endpoint, &format).unwrap();

    // event_loop.play(voice_id);

    // let mut samples_left = reader.len() as usize;

    // let mut append_data = |voice_id: &mut cpal::VoiceId| {
    //     match voice_id.append_data(samples_left) {
    //         cpal::UnknownTypeBuffer::I16(mut wrapped_buf) => {
    //             // We cannot rely on Rust's autoderef here, because we want to
    //             // call .len() on the buffer, which would cause a deref() of the
    //             // buffer, not a deref_mut(), and cpal's deref() implementation
    //             // is to panic.
    //             let buf: &mut [i16] = &mut *wrapped_buf;
    //             for (dst, src) in buf.iter_mut().zip(reader.samples::<i16>()) {
    //                 *dst = src.unwrap();
    //             }
    //             samples_left -= buf.len();
    //         }
    //         cpal::UnknownTypeBuffer::F32(mut wrapped_buf) => {
    //             let buf: &mut [f32] = &mut *wrapped_buf;
    //             for (dst, src) in buf.iter_mut().zip(reader.samples::<f32>()) {
    //                 *dst = src.unwrap();
    //             }
    //             samples_left -= buf.len();
    //         }
    //         _ => unreachable!()
    //     }

    //     // Loop again if there are samples left.
    //     samples_left > 0
    // };

    // // The voice must have some data before playing for the first time.
    // let mut has_more = append_data(&mut voice_id);
    // voice_id.play();

    // // Then we keep providing new data until the end of the audio.
    // while has_more {
    //     has_more = append_data(&mut voice_id);
    // }

    // // Wait for playback to complete.
    // while voice_id.underflowed() {
    //     thread::yield_now();
    // }
}

// fn matches_format(format: &cpal::Format, spec: &hound::WavSpec) -> bool {
//     let cpal::SamplesRate(sample_rate) = format.samples_rate;
//     if sample_rate != spec.sample_rate {
//         return false
//     }

//     if format.channels.len() != spec.channels as usize {
//         return false
//     }

//     let data_type = match (spec.bits_per_sample, spec.sample_format) {
//         (16, hound::SampleFormat::Int) => Some(cpal::SampleFormat::I16),
//         (32, hound::SampleFormat::Float) => Some(cpal::SampleFormat::F32),
//         _ => None
//     };

//     if Some(format.data_type) != data_type {
//         return false
//     }

//     // If the sample rate, channel count, and sample format match, then we can
//     // play back the file in this format.
//     true
// }
