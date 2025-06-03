#[doc = "Register `SCAR_PRG1` reader"]
pub type R = crate::R<ScarPrg1Spec>;
#[doc = "Register `SCAR_PRG1` writer"]
pub type W = crate::W<ScarPrg1Spec>;
#[doc = "Field `SEC_AREA_START1` reader - Bank 1 lowest secure protected address configuration"]
pub type SecAreaStart1R = crate::FieldReader<u16>;
#[doc = "Field `SEC_AREA_START1` writer - Bank 1 lowest secure protected address configuration"]
pub type SecAreaStart1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SEC_AREA_END1` reader - Bank 1 highest secure protected address configuration"]
pub type SecAreaEnd1R = crate::FieldReader<u16>;
#[doc = "Field `SEC_AREA_END1` writer - Bank 1 highest secure protected address configuration"]
pub type SecAreaEnd1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMES1` reader - Bank 1 secure protected erase enable option configuration bit"]
pub type Dmes1R = crate::BitReader;
#[doc = "Field `DMES1` writer - Bank 1 secure protected erase enable option configuration bit"]
pub type Dmes1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Bank 1 lowest secure protected address configuration"]
    #[inline(always)]
    pub fn sec_area_start1(&self) -> SecAreaStart1R {
        SecAreaStart1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 1 highest secure protected address configuration"]
    #[inline(always)]
    pub fn sec_area_end1(&self) -> SecAreaEnd1R {
        SecAreaEnd1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 1 secure protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmes1(&self) -> Dmes1R {
        Dmes1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bank 1 lowest secure protected address configuration"]
    #[inline(always)]
    pub fn sec_area_start1(&mut self) -> SecAreaStart1W<ScarPrg1Spec> {
        SecAreaStart1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Bank 1 highest secure protected address configuration"]
    #[inline(always)]
    pub fn sec_area_end1(&mut self) -> SecAreaEnd1W<ScarPrg1Spec> {
        SecAreaEnd1W::new(self, 16)
    }
    #[doc = "Bit 31 - Bank 1 secure protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmes1(&mut self) -> Dmes1W<ScarPrg1Spec> {
        Dmes1W::new(self, 31)
    }
}
#[doc = "FLASH secure address for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scar_prg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_prg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScarPrg1Spec;
impl crate::RegisterSpec for ScarPrg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scar_prg1::R`](R) reader structure"]
impl crate::Readable for ScarPrg1Spec {}
#[doc = "`write(|w| ..)` method takes [`scar_prg1::W`](W) writer structure"]
impl crate::Writable for ScarPrg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCAR_PRG1 to value 0"]
impl crate::Resettable for ScarPrg1Spec {}
