#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BRR_0_3` reader - DIV_Fraction"]
pub type Brr0_3R = crate::FieldReader;
#[doc = "Field `BRR_0_3` writer - DIV_Fraction"]
pub type Brr0_3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BRR_4_15` reader - DIV_Mantissa"]
pub type Brr4_15R = crate::FieldReader<u16>;
#[doc = "Field `BRR_4_15` writer - DIV_Mantissa"]
pub type Brr4_15W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - DIV_Fraction"]
    #[inline(always)]
    pub fn brr_0_3(&self) -> Brr0_3R {
        Brr0_3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - DIV_Mantissa"]
    #[inline(always)]
    pub fn brr_4_15(&self) -> Brr4_15R {
        Brr4_15R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DIV_Fraction"]
    #[inline(always)]
    pub fn brr_0_3(&mut self) -> Brr0_3W<BrrSpec> {
        Brr0_3W::new(self, 0)
    }
    #[doc = "Bits 4:15 - DIV_Mantissa"]
    #[inline(always)]
    pub fn brr_4_15(&mut self) -> Brr4_15W<BrrSpec> {
        Brr4_15W::new(self, 4)
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BrrSpec {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BrrSpec {}
