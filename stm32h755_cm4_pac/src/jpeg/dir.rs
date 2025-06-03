#[doc = "Register `DIR` writer"]
pub type W = crate::W<DirSpec>;
#[doc = "Field `DATAIN` writer - Data Input FIFO Input FIFO data register."]
pub type DatainW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Data Input FIFO Input FIFO data register."]
    #[inline(always)]
    pub fn datain(&mut self) -> DatainW<DirSpec> {
        DatainW::new(self, 0)
    }
}
#[doc = "JPEG data input register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirSpec;
impl crate::RegisterSpec for DirSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dir::W`](W) writer structure"]
impl crate::Writable for DirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DirSpec {}
