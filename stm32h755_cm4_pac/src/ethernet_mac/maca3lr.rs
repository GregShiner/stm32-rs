#[doc = "Register `MACA3LR` reader"]
pub type R = crate::R<Maca3lrSpec>;
#[doc = "Register `MACA3LR` writer"]
pub type W = crate::W<Maca3lrSpec>;
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
    pub fn addrlo(&mut self) -> AddrloW<Maca3lrSpec> {
        AddrloW::new(self, 0)
    }
}
#[doc = "Address 3 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca3lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca3lrSpec;
impl crate::RegisterSpec for Maca3lrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca3lr::R`](R) reader structure"]
impl crate::Readable for Maca3lrSpec {}
#[doc = "`write(|w| ..)` method takes [`maca3lr::W`](W) writer structure"]
impl crate::Writable for Maca3lrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACA3LR to value 0xffff_ffff"]
impl crate::Resettable for Maca3lrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
