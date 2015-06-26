////////////////////////////////////////////////////////////////////////////////
//                                                                            //
//  Copyright (c) 2015 Mohd Tarmizi Mohd Affandi                              //
//                                                                            //
//  Permission is hereby granted, free of charge, to any person obtaining a   //
//  copy of this software and associated documentation files (the             //
//  "Software"), to deal in the Software without restriction, including       //
//  without limitation the rights to use, copy, modify, merge, publish,       //
//  distribute, sublicense, and/or sell copies of the Software, and to        //
//  permit persons to whom the Software is furnished to do so, subject to     //
//  the following conditions:                                                 //
//                                                                            //
//  The above copyright notice and this permission notice shall be included   //
//  in all copies or substantial portions of the Software.                    //
//                                                                            //
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS   //
//  OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF                //
//  MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.    //
//  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY      //
//  CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,      //
//  TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE         //
//  SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.                    //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};

pub struct Transport<'a> {
    inner: &'a TcpListener,
}

impl<'a> Transport<'a> {
    pub fn from_tcp(listener: &'a TcpListener) -> Self {
        Transport { inner: listener }
    }

    pub fn accept(&mut self) -> io::Result<Socket> {
        let (stream, _) = try!(self.inner.accept());
        Ok(Socket { inner: stream })
    }
}

pub struct Socket {
    inner: TcpStream,
}

impl Socket {
    pub fn peer(&self) -> io::Result<String> {
        match try!(self.inner.peer_addr()) {
            SocketAddr::V4(addr) => Ok(addr.ip().to_string()),
            SocketAddr::V6(addr) => Ok(addr.ip().to_string()),
        }
    }
}

impl<'a> Read for &'a Socket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&self.inner).read(buf)
    }
}

impl<'a> Write for &'a Socket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&self.inner).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        (&self.inner).flush()
    }
}

impl Read for Socket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&*self).read(buf)
    }
}

impl Write for Socket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&*self).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        (&*self).flush()
    }
}
