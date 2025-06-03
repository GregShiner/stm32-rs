#[doc = "Register `UR4` reader"]
pub type R = crate::R<Ur4Spec>;
#[doc = "Register `UR4` writer"]
pub type W = crate::W<Ur4Spec>;
#[doc = "Field `BCM4_ADD1` reader - Mass Erase Protected Area Disabled for bank 1"]
pub type Bcm4Add1R = crate::FieldReader<u16>;
#[doc = "Field `BCM4_ADD1` writer - Mass Erase Protected Area Disabled for bank 1"]
pub type Bcm4Add1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MEPAD_1` reader - Boot Cortex-M4 Address 1"]
pub type Mepad1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Mass Erase Protected Area Disabled for bank 1"]
    #[inline(always)]
    pub fn bcm4_add1(&self) -> Bcm4Add1R {
        Bcm4Add1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Boot Cortex-M4 Address 1"]
    #[inline(always)]
    pub fn mepad_1(&self) -> Mepad1R {
        Mepad1R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mass Erase Protected Area Disabled for bank 1"]
    #[inline(always)]
    pub fn bcm4_add1(&mut self) -> Bcm4Add1W<Ur4Spec> {
        Bcm4Add1W::new(self, 0)
    }
}
#[doc = "SYSCFG user register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ur4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur4Spec;
impl crate::RegisterSpec for Ur4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur4::R`](R) reader structure"]
impl crate::Readable for Ur4Spec {}
#[doc = "`write(|w| ..)` method takes [`ur4::W`](W) writer structure"]
impl crate::Writable for Ur4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UR4 to value 0"]
impl crate::Resettable for Ur4Spec {}
