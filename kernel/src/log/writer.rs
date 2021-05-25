/*! Kernel logger writer */

use alloc::vec::Vec;
use core::{
    fmt,
    str
};

use shared::logger::writers::LoggerWriter;

/**
 * Line-buffer level writer which relies on another `LoggerWriter` to flush
 */
pub(super) struct BufferedWriter<W>
    where W: LoggerWriter {
    m_buffer: Option<Vec<u8>>,
    m_buffered: bool,
    m_writer: W
}

impl<W> BufferedWriter<W> where W: LoggerWriter {
    /**
     * Enables the line-buffering with the given size
     */
    pub fn enable_buffering(&mut self,
                            use_previous_buffer_if_any: bool,
                            buffer_size: usize) {
        if !use_previous_buffer_if_any || self.m_buffer.is_none() {
            self.m_buffer = Some(Vec::with_capacity(buffer_size));
        }
        self.m_buffered = true;
    }

    /*
     * Disables the line-buffering and destroys the buffer if `keep_buffer`
     * is false
     */
    //pub fn disable_buffering(&mut self, keep_buffer: bool) {
    //    self.m_buffered = false;
    //    if !keep_buffer {
    //        /* overwriting with <None> the field throws the <drop> call for it */
    //        self.m_buffer = None;
    //    }
    //}

    /**
     * Writes the given `str_chunk` into the buffer and flushes to the back
     * writer when encounters `\n`
     */
    fn write_buffered(&mut self, str_chunk: &str) -> fmt::Result {
        /* extend the buffer if the remaining capacity doesn't suffice */
        let need_reservation = if !self.can_store(str_chunk) {
            true
        } else {
            false
        };

        if let Some(ref mut buffer) = self.m_buffer {
            /* extend the buffer if the remaining capacity doesn't suffice */
            if need_reservation {
                buffer.reserve(str_chunk.len());
            }

            /* iterate each byte into the given string and put it into the buffer */
            for byte in str_chunk.as_bytes() {
                let byte = *byte;
                buffer.push(byte);

                /* catch ASCII new-line to flush the buffer */
                if byte == b'\n' {
                    /* flush the buffer */
                    if let Ok(utf8_str) = str::from_utf8(buffer.as_slice()) {
                        if let Err(err) = self.m_writer.write_str(utf8_str) {
                            return Err(err);
                        } else {
                            /* reset the buffer */
                            buffer.truncate(0);
                        }
                    } else {
                        return Err(fmt::Error);
                    }
                }
            }
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }

    /**
     * Returns whether the buffer can hold `s` without re-allocations
     */
    fn can_store(&self, s: &str) -> bool {
        if let Some(ref buffer) = self.m_buffer {
            buffer.capacity() - buffer.len() < s.len()
        } else {
            false
        }
    }
}

impl<W> LoggerWriter for BufferedWriter<W> where W: LoggerWriter {
    fn new() -> Self {
        Self { m_buffer: None,
               m_buffered: false,
               m_writer: W::new() }
    }
}

impl<W> fmt::Write for BufferedWriter<W> where W: LoggerWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.m_buffered {
            self.write_buffered(s)
        } else {
            self.m_writer.write_str(s)
        }
    }
}
