use std::{
    borrow::Borrow,
    io::{Read, Write},
};

#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    key_index: usize,
}

impl<'a> Xorcism<'a> {
    pub fn new<Key>(key: &'a Key) -> Self
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        Self {
            key: key.as_ref(),
            key_index: 0,
        }
    }

    fn get_byte_key(&mut self) -> u8 {
        let byte = self.key[self.key_index];
        self.key_index = (self.key_index + 1) % self.key.len();
        byte
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            .for_each(|byte| *byte ^= self.get_byte_key());
    }

    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8>
    where
        Data: IntoIterator,
        Data::Item: Borrow<u8>,
    {
        data.into_iter()
            .map(|byte| byte.borrow() ^ self.get_byte_key())
    }

    pub fn reader<R: Read>(self, r: R) -> XorcismReader<'a, R> {
        XorcismReader {
            xorcism: self,
            reader: r,
        }
    }

    pub fn writer<W: Write>(self, w: W) -> XorcismWriter<'a, W> {
        XorcismWriter {
            xorcism: self,
            writer: w,
        }
    }
}

pub struct XorcismReader<'a, R: Read> {
    xorcism: Xorcism<'a>,
    reader: R,
}

impl<'a, R: Read> Read for XorcismReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut data = vec![];
        self.reader.read_to_end(&mut data)?;
        let len = data.len();

        buf.iter_mut()
            .zip(self.xorcism.munge(data))
            .for_each(|(slot, data)| *slot = data);
        Ok(len)
    }
}

pub struct XorcismWriter<'a, W: Write> {
    xorcism: Xorcism<'a>,
    writer: W,
}

impl<'a, W: Write> Write for XorcismWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len();
        let data = self.xorcism.munge(buf);
        for d in data {
            self.writer.write_all(&[d])?;
        }
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
