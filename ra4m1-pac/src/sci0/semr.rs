#[doc = "Register `SEMR` reader"]
pub type R = crate::R<SEMR_SPEC>;
#[doc = "Register `SEMR` writer"]
pub type W = crate::W<SEMR_SPEC>;
#[doc = "Field `BRME` reader - Bit Rate Modulation Enable"]
pub type BRME_R = crate::BitReader<BRME_A>;
#[doc = "Bit Rate Modulation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRME_A {
    #[doc = "0: Bit rate modulation function is disabled."]
    _0 = 0,
    #[doc = "1: Bit rate modulation function is enabled."]
    _1 = 1,
}
impl From<BRME_A> for bool {
    #[inline(always)]
    fn from(variant: BRME_A) -> Self {
        variant as u8 != 0
    }
}
impl BRME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRME_A {
        match self.bits {
            false => BRME_A::_0,
            true => BRME_A::_1,
        }
    }
    #[doc = "Bit rate modulation function is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRME_A::_0
    }
    #[doc = "Bit rate modulation function is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRME_A::_1
    }
}
#[doc = "Field `BRME` writer - Bit Rate Modulation Enable"]
pub type BRME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BRME_A>;
impl<'a, REG, const O: u8> BRME_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit rate modulation function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BRME_A::_0)
    }
    #[doc = "Bit rate modulation function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BRME_A::_1)
    }
}
#[doc = "Field `ABCSE` reader - Asynchronous Mode Extended Base Clock Select 1 (Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)"]
pub type ABCSE_R = crate::BitReader<ABCSE_A>;
#[doc = "Asynchronous Mode Extended Base Clock Select 1 (Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABCSE_A {
    #[doc = "0: Clock cycles for 1-bit period is decided with combination between BGDM and ABCS in SEMR."]
    _0 = 0,
    #[doc = "1: Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator."]
    _1 = 1,
}
impl From<ABCSE_A> for bool {
    #[inline(always)]
    fn from(variant: ABCSE_A) -> Self {
        variant as u8 != 0
    }
}
impl ABCSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABCSE_A {
        match self.bits {
            false => ABCSE_A::_0,
            true => ABCSE_A::_1,
        }
    }
    #[doc = "Clock cycles for 1-bit period is decided with combination between BGDM and ABCS in SEMR."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABCSE_A::_0
    }
    #[doc = "Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABCSE_A::_1
    }
}
#[doc = "Field `ABCSE` writer - Asynchronous Mode Extended Base Clock Select 1 (Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)"]
pub type ABCSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ABCSE_A>;
impl<'a, REG, const O: u8> ABCSE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock cycles for 1-bit period is decided with combination between BGDM and ABCS in SEMR."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ABCSE_A::_0)
    }
    #[doc = "Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ABCSE_A::_1)
    }
}
#[doc = "Field `ABCS` reader - Asynchronous Mode Base Clock Select (Valid only in asynchronous mode)"]
pub type ABCS_R = crate::BitReader<ABCS_A>;
#[doc = "Asynchronous Mode Base Clock Select (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABCS_A {
    #[doc = "0: Selects 16 base clock cycles for 1-bit period."]
    _0 = 0,
    #[doc = "1: Selects 8 base clock cycles for 1-bit period."]
    _1 = 1,
}
impl From<ABCS_A> for bool {
    #[inline(always)]
    fn from(variant: ABCS_A) -> Self {
        variant as u8 != 0
    }
}
impl ABCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABCS_A {
        match self.bits {
            false => ABCS_A::_0,
            true => ABCS_A::_1,
        }
    }
    #[doc = "Selects 16 base clock cycles for 1-bit period."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABCS_A::_0
    }
    #[doc = "Selects 8 base clock cycles for 1-bit period."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABCS_A::_1
    }
}
#[doc = "Field `ABCS` writer - Asynchronous Mode Base Clock Select (Valid only in asynchronous mode)"]
pub type ABCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ABCS_A>;
impl<'a, REG, const O: u8> ABCS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects 16 base clock cycles for 1-bit period."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ABCS_A::_0)
    }
    #[doc = "Selects 8 base clock cycles for 1-bit period."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ABCS_A::_1)
    }
}
#[doc = "Field `NFEN` reader - Digital Noise Filter Function Enable (The NFEN bit should be 0 without simple I2C mode and asynchronous mode.) In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input."]
pub type NFEN_R = crate::BitReader<NFEN_A>;
#[doc = "Digital Noise Filter Function Enable (The NFEN bit should be 0 without simple I2C mode and asynchronous mode.) In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFEN_A {
    #[doc = "0: Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is disabled."]
    _0 = 0,
    #[doc = "1: Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is enabled."]
    _1 = 1,
}
impl From<NFEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NFEN_A {
        match self.bits {
            false => NFEN_A::_0,
            true => NFEN_A::_1,
        }
    }
    #[doc = "Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFEN_A::_0
    }
    #[doc = "Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFEN_A::_1
    }
}
#[doc = "Field `NFEN` writer - Digital Noise Filter Function Enable (The NFEN bit should be 0 without simple I2C mode and asynchronous mode.) In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input."]
pub type NFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NFEN_A>;
impl<'a, REG, const O: u8> NFEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NFEN_A::_0)
    }
    #[doc = "Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NFEN_A::_1)
    }
}
#[doc = "Field `BGDM` reader - Baud Rate Generator Double-Speed Mode Select (Only valid the CKE\\[1\\]
bit in SCR is 0 in asynchronous mode)."]
pub type BGDM_R = crate::BitReader<BGDM_A>;
#[doc = "Baud Rate Generator Double-Speed Mode Select (Only valid the CKE\\[1\\]
bit in SCR is 0 in asynchronous mode).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGDM_A {
    #[doc = "0: Baud rate generator outputs the clock with normal frequency."]
    _0 = 0,
    #[doc = "1: Baud rate generator outputs the clock with doubled frequency."]
    _1 = 1,
}
impl From<BGDM_A> for bool {
    #[inline(always)]
    fn from(variant: BGDM_A) -> Self {
        variant as u8 != 0
    }
}
impl BGDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BGDM_A {
        match self.bits {
            false => BGDM_A::_0,
            true => BGDM_A::_1,
        }
    }
    #[doc = "Baud rate generator outputs the clock with normal frequency."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGDM_A::_0
    }
    #[doc = "Baud rate generator outputs the clock with doubled frequency."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGDM_A::_1
    }
}
#[doc = "Field `BGDM` writer - Baud Rate Generator Double-Speed Mode Select (Only valid the CKE\\[1\\]
bit in SCR is 0 in asynchronous mode)."]
pub type BGDM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BGDM_A>;
impl<'a, REG, const O: u8> BGDM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Baud rate generator outputs the clock with normal frequency."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BGDM_A::_0)
    }
    #[doc = "Baud rate generator outputs the clock with doubled frequency."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BGDM_A::_1)
    }
}
#[doc = "Field `RXDESEL` reader - Asynchronous Start Bit Edge Detection Select (Valid only in asynchronous mode)"]
pub type RXDESEL_R = crate::BitReader<RXDESEL_A>;
#[doc = "Asynchronous Start Bit Edge Detection Select (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDESEL_A {
    #[doc = "0: The low level on the RXDn pin is detected as the start bit."]
    _0 = 0,
    #[doc = "1: A falling edge on the RXDn pin is detected as the start bit."]
    _1 = 1,
}
impl From<RXDESEL_A> for bool {
    #[inline(always)]
    fn from(variant: RXDESEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDESEL_A {
        match self.bits {
            false => RXDESEL_A::_0,
            true => RXDESEL_A::_1,
        }
    }
    #[doc = "The low level on the RXDn pin is detected as the start bit."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDESEL_A::_0
    }
    #[doc = "A falling edge on the RXDn pin is detected as the start bit."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDESEL_A::_1
    }
}
#[doc = "Field `RXDESEL` writer - Asynchronous Start Bit Edge Detection Select (Valid only in asynchronous mode)"]
pub type RXDESEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RXDESEL_A>;
impl<'a, REG, const O: u8> RXDESEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The low level on the RXDn pin is detected as the start bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RXDESEL_A::_0)
    }
    #[doc = "A falling edge on the RXDn pin is detected as the start bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RXDESEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - Bit Rate Modulation Enable"]
    #[inline(always)]
    pub fn brme(&self) -> BRME_R {
        BRME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Asynchronous Mode Extended Base Clock Select 1 (Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)"]
    #[inline(always)]
    pub fn abcse(&self) -> ABCSE_R {
        ABCSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous Mode Base Clock Select (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn abcs(&self) -> ABCS_R {
        ABCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Digital Noise Filter Function Enable (The NFEN bit should be 0 without simple I2C mode and asynchronous mode.) In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input."]
    #[inline(always)]
    pub fn nfen(&self) -> NFEN_R {
        NFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Baud Rate Generator Double-Speed Mode Select (Only valid the CKE\\[1\\]
bit in SCR is 0 in asynchronous mode)."]
    #[inline(always)]
    pub fn bgdm(&self) -> BGDM_R {
        BGDM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Asynchronous Start Bit Edge Detection Select (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn rxdesel(&self) -> RXDESEL_R {
        RXDESEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Bit Rate Modulation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brme(&mut self) -> BRME_W<SEMR_SPEC, 2> {
        BRME_W::new(self)
    }
    #[doc = "Bit 3 - Asynchronous Mode Extended Base Clock Select 1 (Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)"]
    #[inline(always)]
    #[must_use]
    pub fn abcse(&mut self) -> ABCSE_W<SEMR_SPEC, 3> {
        ABCSE_W::new(self)
    }
    #[doc = "Bit 4 - Asynchronous Mode Base Clock Select (Valid only in asynchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn abcs(&mut self) -> ABCS_W<SEMR_SPEC, 4> {
        ABCS_W::new(self)
    }
    #[doc = "Bit 5 - Digital Noise Filter Function Enable (The NFEN bit should be 0 without simple I2C mode and asynchronous mode.) In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input."]
    #[inline(always)]
    #[must_use]
    pub fn nfen(&mut self) -> NFEN_W<SEMR_SPEC, 5> {
        NFEN_W::new(self)
    }
    #[doc = "Bit 6 - Baud Rate Generator Double-Speed Mode Select (Only valid the CKE\\[1\\]
bit in SCR is 0 in asynchronous mode)."]
    #[inline(always)]
    #[must_use]
    pub fn bgdm(&mut self) -> BGDM_W<SEMR_SPEC, 6> {
        BGDM_W::new(self)
    }
    #[doc = "Bit 7 - Asynchronous Start Bit Edge Detection Select (Valid only in asynchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn rxdesel(&mut self) -> RXDESEL_W<SEMR_SPEC, 7> {
        RXDESEL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Serial Extended Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`semr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`semr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEMR_SPEC;
impl crate::RegisterSpec for SEMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`semr::R`](R) reader structure"]
impl crate::Readable for SEMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`semr::W`](W) writer structure"]
impl crate::Writable for SEMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEMR to value 0"]
impl crate::Resettable for SEMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
