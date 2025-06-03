#[doc = "Register `PRAR_CUR2` reader"]
pub type R = crate::R<PrarCur2Spec>;
#[doc = "Field `PROT_AREA_START2` reader - Bank 2 lowest PCROP protected address"]
pub type ProtAreaStart2R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_END2` reader - Bank 2 highest PCROP protected address"]
pub type ProtAreaEnd2R = crate::FieldReader<u16>;
#[doc = "Field `DMEP2` reader - Bank 2 PCROP protected erase enable option status bit"]
pub type Dmep2R = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - Bank 2 lowest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_start2(&self) -> ProtAreaStart2R {
        ProtAreaStart2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 2 highest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_end2(&self) -> ProtAreaEnd2R {
        ProtAreaEnd2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 2 PCROP protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmep2(&self) -> Dmep2R {
        Dmep2R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "FLASH protection address for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`prar_cur2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrarCur2Spec;
impl crate::RegisterSpec for PrarCur2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prar_cur2::R`](R) reader structure"]
impl crate::Readable for PrarCur2Spec {}
#[doc = "`reset()` method sets PRAR_CUR2 to value 0"]
impl crate::Resettable for PrarCur2Spec {}
