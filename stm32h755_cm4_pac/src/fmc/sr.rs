#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `IRS` reader - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
pub type IrsR = crate::BitReader;
#[doc = "Field `IRS` writer - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
pub type IrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILS` reader - Interrupt high-level status The flag is set by hardware and reset by software."]
pub type IlsR = crate::BitReader;
#[doc = "Field `ILS` writer - Interrupt high-level status The flag is set by hardware and reset by software."]
pub type IlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFS` reader - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
pub type IfsR = crate::BitReader;
#[doc = "Field `IFS` writer - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
pub type IfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREN` reader - Interrupt rising edge detection enable bit"]
pub type IrenR = crate::BitReader;
#[doc = "Field `IREN` writer - Interrupt rising edge detection enable bit"]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILEN` reader - Interrupt high-level detection enable bit"]
pub type IlenR = crate::BitReader;
#[doc = "Field `ILEN` writer - Interrupt high-level detection enable bit"]
pub type IlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFEN` reader - Interrupt falling edge detection enable bit"]
pub type IfenR = crate::BitReader;
#[doc = "Field `IFEN` writer - Interrupt falling edge detection enable bit"]
pub type IfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEMPT` reader - FIFO empty. Read-only bit that provides the status of the FIFO"]
pub type FemptR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
    #[inline(always)]
    pub fn irs(&self) -> IrsR {
        IrsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt high-level status The flag is set by hardware and reset by software."]
    #[inline(always)]
    pub fn ils(&self) -> IlsR {
        IlsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
    #[inline(always)]
    pub fn ifs(&self) -> IfsR {
        IfsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable bit"]
    #[inline(always)]
    pub fn ilen(&self) -> IlenR {
        IlenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable bit"]
    #[inline(always)]
    pub fn ifen(&self) -> IfenR {
        IfenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO empty. Read-only bit that provides the status of the FIFO"]
    #[inline(always)]
    pub fn fempt(&self) -> FemptR {
        FemptR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
    #[inline(always)]
    pub fn irs(&mut self) -> IrsW<SrSpec> {
        IrsW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt high-level status The flag is set by hardware and reset by software."]
    #[inline(always)]
    pub fn ils(&mut self) -> IlsW<SrSpec> {
        IlsW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
    #[inline(always)]
    pub fn ifs(&mut self) -> IfsW<SrSpec> {
        IfsW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn iren(&mut self) -> IrenW<SrSpec> {
        IrenW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable bit"]
    #[inline(always)]
    pub fn ilen(&mut self) -> IlenW<SrSpec> {
        IlenW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable bit"]
    #[inline(always)]
    pub fn ifen(&mut self) -> IfenW<SrSpec> {
        IfenW::new(self, 5)
    }
}
#[doc = "This register contains information about the FIFO status and interrupt. The FMC features a FIFO that is used when writing to memories to transfer up to 16 words of data.This is used to quickly write to the FIFO and free the AXI bus for transactions to peripherals other than the FMC, while the FMC is draining its FIFO into the memory. One of these register bits indicates the status of the FIFO, for ECC purposes.The ECC is calculated while the data are written to the memory. To read the correct ECC, the software must consequently wait until the FIFO is empty.\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0x40"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x40;
}
