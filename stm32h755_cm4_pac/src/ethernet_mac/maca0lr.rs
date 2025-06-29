#[doc = "Register `MACA0LR` reader"]
pub type R = crate::R<Maca0lrSpec>;
#[doc = "Register `MACA0LR` writer"]
pub type W = crate::W<Maca0lrSpec>;
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
    pub fn addrlo(&mut self) -> AddrloW<Maca0lrSpec> {
        AddrloW::new(self, 0)
    }
}
#[doc = "Address 0 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca0lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca0lrSpec;
impl crate::RegisterSpec for Maca0lrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca0lr::R`](R) reader structure"]
impl crate::Readable for Maca0lrSpec {}
#[doc = "`write(|w| ..)` method takes [`maca0lr::W`](W) writer structure"]
impl crate::Writable for Maca0lrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACA0LR to value 0xffff_ffff"]
impl crate::Resettable for Maca0lrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
