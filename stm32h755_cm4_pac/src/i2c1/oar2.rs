#[doc = "Register `OAR2` reader"]
pub type R = crate::R<Oar2Spec>;
#[doc = "Register `OAR2` writer"]
pub type W = crate::W<Oar2Spec>;
#[doc = "Field `OA2` reader - Interface address bits 7:1 of address Note: These bits can be written only when OA2EN=0."]
pub type Oa2R = crate::FieldReader;
#[doc = "Field `OA2` writer - Interface address bits 7:1 of address Note: These bits can be written only when OA2EN=0."]
pub type Oa2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OA2MSK` reader - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
pub type Oa2mskR = crate::FieldReader;
#[doc = "Field `OA2MSK` writer - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
pub type Oa2mskW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OA2EN` reader - Own Address 2 enable"]
pub type Oa2enR = crate::BitReader;
#[doc = "Field `OA2EN` writer - Own Address 2 enable"]
pub type Oa2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:7 - Interface address bits 7:1 of address Note: These bits can be written only when OA2EN=0."]
    #[inline(always)]
    pub fn oa2(&self) -> Oa2R {
        Oa2R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
    #[inline(always)]
    pub fn oa2msk(&self) -> Oa2mskR {
        Oa2mskR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&self) -> Oa2enR {
        Oa2enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address bits 7:1 of address Note: These bits can be written only when OA2EN=0."]
    #[inline(always)]
    pub fn oa2(&mut self) -> Oa2W<Oar2Spec> {
        Oa2W::new(self, 1)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
    #[inline(always)]
    pub fn oa2msk(&mut self) -> Oa2mskW<Oar2Spec> {
        Oa2mskW::new(self, 8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&mut self) -> Oa2enW<Oar2Spec> {
        Oa2enW::new(self, 15)
    }
}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK.\n\nYou can [`read`](crate::Reg::read) this register and get [`oar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oar2Spec;
impl crate::RegisterSpec for Oar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar2::R`](R) reader structure"]
impl crate::Readable for Oar2Spec {}
#[doc = "`write(|w| ..)` method takes [`oar2::W`](W) writer structure"]
impl crate::Writable for Oar2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OAR2 to value 0"]
impl crate::Resettable for Oar2Spec {}
