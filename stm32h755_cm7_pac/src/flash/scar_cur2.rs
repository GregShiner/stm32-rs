#[doc = "Register `SCAR_CUR2` reader"]
pub type R = crate::R<ScarCur2Spec>;
#[doc = "Register `SCAR_CUR2` writer"]
pub type W = crate::W<ScarCur2Spec>;
#[doc = "Field `SEC_AREA_START2` reader - Bank 2 lowest secure protected address"]
pub type SecAreaStart2R = crate::FieldReader<u16>;
#[doc = "Field `SEC_AREA_START2` writer - Bank 2 lowest secure protected address"]
pub type SecAreaStart2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SEC_AREA_END2` reader - Bank 2 highest secure protected address"]
pub type SecAreaEnd2R = crate::FieldReader<u16>;
#[doc = "Field `SEC_AREA_END2` writer - Bank 2 highest secure protected address"]
pub type SecAreaEnd2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMES2` reader - Bank 2 secure protected erase enable option status bit"]
pub type Dmes2R = crate::BitReader;
#[doc = "Field `DMES2` writer - Bank 2 secure protected erase enable option status bit"]
pub type Dmes2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Bank 2 lowest secure protected address"]
    #[inline(always)]
    pub fn sec_area_start2(&self) -> SecAreaStart2R {
        SecAreaStart2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 2 highest secure protected address"]
    #[inline(always)]
    pub fn sec_area_end2(&self) -> SecAreaEnd2R {
        SecAreaEnd2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 2 secure protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmes2(&self) -> Dmes2R {
        Dmes2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bank 2 lowest secure protected address"]
    #[inline(always)]
    pub fn sec_area_start2(&mut self) -> SecAreaStart2W<ScarCur2Spec> {
        SecAreaStart2W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Bank 2 highest secure protected address"]
    #[inline(always)]
    pub fn sec_area_end2(&mut self) -> SecAreaEnd2W<ScarCur2Spec> {
        SecAreaEnd2W::new(self, 16)
    }
    #[doc = "Bit 31 - Bank 2 secure protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmes2(&mut self) -> Dmes2W<ScarCur2Spec> {
        Dmes2W::new(self, 31)
    }
}
#[doc = "FLASH secure address for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scar_cur2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_cur2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScarCur2Spec;
impl crate::RegisterSpec for ScarCur2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scar_cur2::R`](R) reader structure"]
impl crate::Readable for ScarCur2Spec {}
#[doc = "`write(|w| ..)` method takes [`scar_cur2::W`](W) writer structure"]
impl crate::Writable for ScarCur2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCAR_CUR2 to value 0"]
impl crate::Resettable for ScarCur2Spec {}
