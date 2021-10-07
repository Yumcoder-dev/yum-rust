// https://github.com/colin-kiegel/rust-derive-builder
mod test {
    #[test]
    fn test_builders() -> Result<(), String> {
        // std example:
        // let f = std::fs::OpenOptions::new()
        //     .read(true)
        //     .write(true)
        //     .create(true)
        //     .append(true)
        //     .open(path)?;
        #[derive(Debug)]
        struct Lorem {
            ipsum: u32,
            // ..
        }
        #[derive(Debug, Clone, Default)]
        struct LoremBuilder {
            ipsum: Option<u32>,
        }
        #[allow(dead_code)]
        impl LoremBuilder {
            pub fn ipsum(&mut self, value: u32) -> &mut Self {
                let mut new = self;
                new.ipsum = Some(value);
                new
            }
            // ...
            fn build(&self) -> Result<Lorem, String> {
                Ok(Lorem {
                    ipsum: Clone::clone(self.ipsum.as_ref().ok_or("ipsum must be initialized")?),
                })
            }
        }

        let x: Lorem = LoremBuilder::default().ipsum(42).build()?;
        println!("{:?}", x);
        Ok(())
    }
}
