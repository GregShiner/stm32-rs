#[doc = "Register `MACPPSCR` reader"]
pub type R = crate::R<MacppscrSpec>;
#[doc = "Register `MACPPSCR` writer"]
pub type W = crate::W<MacppscrSpec>;
#[doc = "Field `PPSCTRL` reader - PPSCTRL"]
pub type PpsctrlR = crate::FieldReader;
#[doc = "Field `PPSCTRL` writer - PPSCTRL"]
pub type PpsctrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPSEN0` reader - PPSEN0"]
pub type Ppsen0R = crate::BitReader;
#[doc = "Field `PPSEN0` writer - PPSEN0"]
pub type Ppsen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGTMODSEL0` reader - TRGTMODSEL0"]
pub type Trgtmodsel0R = crate::FieldReader;
#[doc = "Field `TRGTMODSEL0` writer - TRGTMODSEL0"]
pub type Trgtmodsel0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - PPSCTRL"]
    #[inline(always)]
    pub fn ppsctrl(&self) -> PpsctrlR {
        PpsctrlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - PPSEN0"]
    #[inline(always)]
    pub fn ppsen0(&self) -> Ppsen0R {
        Ppsen0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - TRGTMODSEL0"]
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> Trgtmodsel0R {
        Trgtmodsel0R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PPSCTRL"]
    #[inline(always)]
    pub fn ppsctrl(&mut self) -> PpsctrlW<MacppscrSpec> {
        PpsctrlW::new(self, 0)
    }
    #[doc = "Bit 4 - PPSEN0"]
    #[inline(always)]
    pub fn ppsen0(&mut self) -> Ppsen0W<MacppscrSpec> {
        Ppsen0W::new(self, 4)
    }
    #[doc = "Bits 5:6 - TRGTMODSEL0"]
    #[inline(always)]
    pub fn trgtmodsel0(&mut self) -> Trgtmodsel0W<MacppscrSpec> {
        Trgtmodsel0W::new(self, 5)
    }
}
#[doc = "PPS control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macppscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacppscrSpec;
impl crate::RegisterSpec for MacppscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppscr::R`](R) reader structure"]
impl crate::Readable for MacppscrSpec {}
#[doc = "`write(|w| ..)` method takes [`macppscr::W`](W) writer structure"]
impl crate::Writable for MacppscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACPPSCR to value 0"]
impl crate::Resettable for MacppscrSpec {}
