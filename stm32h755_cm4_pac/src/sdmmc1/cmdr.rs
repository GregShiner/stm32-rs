#[doc = "Register `CMDR` reader"]
pub type R = crate::R<CmdrSpec>;
#[doc = "Register `CMDR` writer"]
pub type W = crate::W<CmdrSpec>;
#[doc = "Field `CMDINDEX` reader - Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
pub type CmdindexR = crate::FieldReader;
#[doc = "Field `CMDINDEX` writer - Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
pub type CmdindexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CMDTRANS` reader - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to the DPSM when the command is sent."]
pub type CmdtransR = crate::BitReader;
#[doc = "Field `CMDTRANS` writer - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to the DPSM when the command is sent."]
pub type CmdtransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSTOP` reader - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues the Abort signal to the DPSM when the command is sent."]
pub type CmdstopR = crate::BitReader;
#[doc = "Field `CMDSTOP` writer - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues the Abort signal to the DPSM when the command is sent."]
pub type CmdstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITRESP` reader - Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
pub type WaitrespR = crate::FieldReader;
#[doc = "Field `WAITRESP` writer - Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
pub type WaitrespW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAITINT` reader - CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode."]
pub type WaitintR = crate::BitReader;
#[doc = "Field `WAITINT` writer - CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode."]
pub type WaitintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITPEND` reader - CPSM Waits for end of data transfer (CmdPend internal signal) from DPSM. This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending a command. WAITPEND is only taken into account when DTMODE = MMC stream data transfer, WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card."]
pub type WaitpendR = crate::BitReader;
#[doc = "Field `WAITPEND` writer - CPSM Waits for end of data transfer (CmdPend internal signal) from DPSM. This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending a command. WAITPEND is only taken into account when DTMODE = MMC stream data transfer, WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card."]
pub type WaitpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPSMEN` reader - Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0."]
pub type CpsmenR = crate::BitReader;
#[doc = "Field `CPSMEN` writer - Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0."]
pub type CpsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTHOLD` reader - Hold new data block transmission and reception in the DPSM. If this bit is set, the DPSM will not move from the Wait_S state to the Send state or from the Wait_R state to the Receive state."]
pub type DtholdR = crate::BitReader;
#[doc = "Field `DTHOLD` writer - Hold new data block transmission and reception in the DPSM. If this bit is set, the DPSM will not move from the Wait_S state to the Send state or from the Wait_R state to the Receive state."]
pub type DtholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTMODE` reader - Select the boot mode procedure to be used. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)"]
pub type BootmodeR = crate::BitReader;
#[doc = "Field `BOOTMODE` writer - Select the boot mode procedure to be used. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)"]
pub type BootmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTEN` reader - Enable boot mode procedure."]
pub type BootenR = crate::BitReader;
#[doc = "Field `BOOTEN` writer - Enable boot mode procedure."]
pub type BootenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSUSPEND` reader - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when response bit BS=0. CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period when response bit DF=1."]
pub type CmdsuspendR = crate::BitReader;
#[doc = "Field `CMDSUSPEND` writer - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when response bit BS=0. CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period when response bit DF=1."]
pub type CmdsuspendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
    #[inline(always)]
    pub fn cmdindex(&self) -> CmdindexR {
        CmdindexR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to the DPSM when the command is sent."]
    #[inline(always)]
    pub fn cmdtrans(&self) -> CmdtransR {
        CmdtransR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues the Abort signal to the DPSM when the command is sent."]
    #[inline(always)]
    pub fn cmdstop(&self) -> CmdstopR {
        CmdstopR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
    #[inline(always)]
    pub fn waitresp(&self) -> WaitrespR {
        WaitrespR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode."]
    #[inline(always)]
    pub fn waitint(&self) -> WaitintR {
        WaitintR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPSM Waits for end of data transfer (CmdPend internal signal) from DPSM. This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending a command. WAITPEND is only taken into account when DTMODE = MMC stream data transfer, WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card."]
    #[inline(always)]
    pub fn waitpend(&self) -> WaitpendR {
        WaitpendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0."]
    #[inline(always)]
    pub fn cpsmen(&self) -> CpsmenR {
        CpsmenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hold new data block transmission and reception in the DPSM. If this bit is set, the DPSM will not move from the Wait_S state to the Send state or from the Wait_R state to the Receive state."]
    #[inline(always)]
    pub fn dthold(&self) -> DtholdR {
        DtholdR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select the boot mode procedure to be used. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)"]
    #[inline(always)]
    pub fn bootmode(&self) -> BootmodeR {
        BootmodeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable boot mode procedure."]
    #[inline(always)]
    pub fn booten(&self) -> BootenR {
        BootenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when response bit BS=0. CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period when response bit DF=1."]
    #[inline(always)]
    pub fn cmdsuspend(&self) -> CmdsuspendR {
        CmdsuspendR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CmdindexW<CmdrSpec> {
        CmdindexW::new(self, 0)
    }
    #[doc = "Bit 6 - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to the DPSM when the command is sent."]
    #[inline(always)]
    pub fn cmdtrans(&mut self) -> CmdtransW<CmdrSpec> {
        CmdtransW::new(self, 6)
    }
    #[doc = "Bit 7 - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). If this bit is set, the CPSM issues the Abort signal to the DPSM when the command is sent."]
    #[inline(always)]
    pub fn cmdstop(&mut self) -> CmdstopW<CmdrSpec> {
        CmdstopW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
    #[inline(always)]
    pub fn waitresp(&mut self) -> WaitrespW<CmdrSpec> {
        WaitrespW::new(self, 8)
    }
    #[doc = "Bit 10 - CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode."]
    #[inline(always)]
    pub fn waitint(&mut self) -> WaitintW<CmdrSpec> {
        WaitintW::new(self, 10)
    }
    #[doc = "Bit 11 - CPSM Waits for end of data transfer (CmdPend internal signal) from DPSM. This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending a command. WAITPEND is only taken into account when DTMODE = MMC stream data transfer, WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card."]
    #[inline(always)]
    pub fn waitpend(&mut self) -> WaitpendW<CmdrSpec> {
        WaitpendW::new(self, 11)
    }
    #[doc = "Bit 12 - Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0."]
    #[inline(always)]
    pub fn cpsmen(&mut self) -> CpsmenW<CmdrSpec> {
        CpsmenW::new(self, 12)
    }
    #[doc = "Bit 13 - Hold new data block transmission and reception in the DPSM. If this bit is set, the DPSM will not move from the Wait_S state to the Send state or from the Wait_R state to the Receive state."]
    #[inline(always)]
    pub fn dthold(&mut self) -> DtholdW<CmdrSpec> {
        DtholdW::new(self, 13)
    }
    #[doc = "Bit 14 - Select the boot mode procedure to be used. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)"]
    #[inline(always)]
    pub fn bootmode(&mut self) -> BootmodeW<CmdrSpec> {
        BootmodeW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable boot mode procedure."]
    #[inline(always)]
    pub fn booten(&mut self) -> BootenW<CmdrSpec> {
        BootenW::new(self, 15)
    }
    #[doc = "Bit 16 - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when response bit BS=0. CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period when response bit DF=1."]
    #[inline(always)]
    pub fn cmdsuspend(&mut self) -> CmdsuspendW<CmdrSpec> {
        CmdsuspendW::new(self, 16)
    }
}
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdrSpec;
impl crate::RegisterSpec for CmdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdr::R`](R) reader structure"]
impl crate::Readable for CmdrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdr::W`](W) writer structure"]
impl crate::Writable for CmdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDR to value 0"]
impl crate::Resettable for CmdrSpec {}
