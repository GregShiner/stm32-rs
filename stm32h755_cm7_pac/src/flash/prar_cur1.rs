#[doc = "Register `PRAR_CUR1` reader"]
pub type R = crate::R<PrarCur1Spec>;
#[doc = "Field `PROT_AREA_START1` reader - Bank 1 lowest PCROP protected address"]
pub type ProtAreaStart1R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_END1` reader - Bank 1 highest PCROP protected address"]
pub type ProtAreaEnd1R = crate::FieldReader<u16>;
#[doc = "Field `DMEP1` reader - Bank 1 PCROP protected erase enable option status bit"]
pub type Dmep1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - Bank 1 lowest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_start1(&self) -> ProtAreaStart1R {
        ProtAreaStart1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 1 highest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_end1(&self) -> ProtAreaEnd1R {
        ProtAreaEnd1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmep1(&self) -> Dmep1R {
        Dmep1R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "FLASH protection address for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`prar_cur1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrarCur1Spec;
impl crate::RegisterSpec for PrarCur1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prar_cur1::R`](R) reader structure"]
impl crate::Readable for PrarCur1Spec {}
#[doc = "`reset()` method sets PRAR_CUR1 to value 0"]
impl crate::Resettable for PrarCur1Spec {}
