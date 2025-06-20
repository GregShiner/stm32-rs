#[doc = "Register `IMSCR` reader"]
pub type R = crate::R<ImscrSpec>;
#[doc = "Register `IMSCR` writer"]
pub type W = crate::W<ImscrSpec>;
#[doc = "Field `INIM` reader - Input FIFO service interrupt mask"]
pub type InimR = crate::BitReader;
#[doc = "Field `INIM` writer - Input FIFO service interrupt mask"]
pub type InimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTIM` reader - Output FIFO service interrupt mask"]
pub type OutimR = crate::BitReader;
#[doc = "Field `OUTIM` writer - Output FIFO service interrupt mask"]
pub type OutimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Input FIFO service interrupt mask"]
    #[inline(always)]
    pub fn inim(&self) -> InimR {
        InimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output FIFO service interrupt mask"]
    #[inline(always)]
    pub fn outim(&self) -> OutimR {
        OutimR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input FIFO service interrupt mask"]
    #[inline(always)]
    pub fn inim(&mut self) -> InimW<ImscrSpec> {
        InimW::new(self, 0)
    }
    #[doc = "Bit 1 - Output FIFO service interrupt mask"]
    #[inline(always)]
    pub fn outim(&mut self) -> OutimW<ImscrSpec> {
        OutimW::new(self, 1)
    }
}
#[doc = "interrupt mask set/clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`imscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImscrSpec;
impl crate::RegisterSpec for ImscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imscr::R`](R) reader structure"]
impl crate::Readable for ImscrSpec {}
#[doc = "`write(|w| ..)` method takes [`imscr::W`](W) writer structure"]
impl crate::Writable for ImscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMSCR to value 0"]
impl crate::Resettable for ImscrSpec {}
