#[doc = "Register `PRAR_PRG1` reader"]
pub type R = crate::R<PrarPrg1Spec>;
#[doc = "Register `PRAR_PRG1` writer"]
pub type W = crate::W<PrarPrg1Spec>;
#[doc = "Field `PROT_AREA_START1` reader - Bank 1 lowest PCROP protected address configuration"]
pub type ProtAreaStart1R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_START1` writer - Bank 1 lowest PCROP protected address configuration"]
pub type ProtAreaStart1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PROT_AREA_END1` reader - Bank 1 highest PCROP protected address configuration"]
pub type ProtAreaEnd1R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_END1` writer - Bank 1 highest PCROP protected address configuration"]
pub type ProtAreaEnd1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMEP1` reader - Bank 1 PCROP protected erase enable option configuration bit"]
pub type Dmep1R = crate::BitReader;
#[doc = "Field `DMEP1` writer - Bank 1 PCROP protected erase enable option configuration bit"]
pub type Dmep1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Bank 1 lowest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_start1(&self) -> ProtAreaStart1R {
        ProtAreaStart1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 1 highest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_end1(&self) -> ProtAreaEnd1R {
        ProtAreaEnd1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmep1(&self) -> Dmep1R {
        Dmep1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bank 1 lowest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_start1(&mut self) -> ProtAreaStart1W<PrarPrg1Spec> {
        ProtAreaStart1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Bank 1 highest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_end1(&mut self) -> ProtAreaEnd1W<PrarPrg1Spec> {
        ProtAreaEnd1W::new(self, 16)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmep1(&mut self) -> Dmep1W<PrarPrg1Spec> {
        Dmep1W::new(self, 31)
    }
}
#[doc = "FLASH protection address for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`prar_prg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prar_prg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrarPrg1Spec;
impl crate::RegisterSpec for PrarPrg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prar_prg1::R`](R) reader structure"]
impl crate::Readable for PrarPrg1Spec {}
#[doc = "`write(|w| ..)` method takes [`prar_prg1::W`](W) writer structure"]
impl crate::Writable for PrarPrg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRAR_PRG1 to value 0"]
impl crate::Resettable for PrarPrg1Spec {}
