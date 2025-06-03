#[doc = "Register `PRAR_PRG2` reader"]
pub type R = crate::R<PrarPrg2Spec>;
#[doc = "Register `PRAR_PRG2` writer"]
pub type W = crate::W<PrarPrg2Spec>;
#[doc = "Field `PROT_AREA_START2` reader - Bank 2 lowest PCROP protected address configuration"]
pub type ProtAreaStart2R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_START2` writer - Bank 2 lowest PCROP protected address configuration"]
pub type ProtAreaStart2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PROT_AREA_END2` reader - Bank 2 highest PCROP protected address configuration"]
pub type ProtAreaEnd2R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_END2` writer - Bank 2 highest PCROP protected address configuration"]
pub type ProtAreaEnd2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMEP2` reader - Bank 2 PCROP protected erase enable option configuration bit"]
pub type Dmep2R = crate::BitReader;
#[doc = "Field `DMEP2` writer - Bank 2 PCROP protected erase enable option configuration bit"]
pub type Dmep2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Bank 2 lowest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_start2(&self) -> ProtAreaStart2R {
        ProtAreaStart2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 2 highest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_end2(&self) -> ProtAreaEnd2R {
        ProtAreaEnd2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 2 PCROP protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmep2(&self) -> Dmep2R {
        Dmep2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bank 2 lowest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_start2(&mut self) -> ProtAreaStart2W<PrarPrg2Spec> {
        ProtAreaStart2W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Bank 2 highest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_end2(&mut self) -> ProtAreaEnd2W<PrarPrg2Spec> {
        ProtAreaEnd2W::new(self, 16)
    }
    #[doc = "Bit 31 - Bank 2 PCROP protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmep2(&mut self) -> Dmep2W<PrarPrg2Spec> {
        Dmep2W::new(self, 31)
    }
}
#[doc = "FLASH protection address for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`prar_prg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prar_prg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrarPrg2Spec;
impl crate::RegisterSpec for PrarPrg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prar_prg2::R`](R) reader structure"]
impl crate::Readable for PrarPrg2Spec {}
#[doc = "`write(|w| ..)` method takes [`prar_prg2::W`](W) writer structure"]
impl crate::Writable for PrarPrg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRAR_PRG2 to value 0"]
impl crate::Resettable for PrarPrg2Spec {}
