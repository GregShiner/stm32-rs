#[doc = "Register `MACA1LR` reader"]
pub type R = crate::R<Maca1lrSpec>;
#[doc = "Register `MACA1LR` writer"]
pub type W = crate::W<Maca1lrSpec>;
#[doc = "Field `ADDRLO` reader - ADDRLO"]
pub type AddrloR = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO` writer - ADDRLO"]
pub type AddrloW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADDRLO"]
    #[inline(always)]
    pub fn addrlo(&self) -> AddrloR {
        AddrloR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADDRLO"]
    #[inline(always)]
    pub fn addrlo(&mut self) -> AddrloW<Maca1lrSpec> {
        AddrloW::new(self, 0)
    }
}
#[doc = "Address 1 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca1lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca1lrSpec;
impl crate::RegisterSpec for Maca1lrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca1lr::R`](R) reader structure"]
impl crate::Readable for Maca1lrSpec {}
#[doc = "`write(|w| ..)` method takes [`maca1lr::W`](W) writer structure"]
impl crate::Writable for Maca1lrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACA1LR to value 0xffff_ffff"]
impl crate::Resettable for Maca1lrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
