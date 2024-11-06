#[doc = "Register `VBTWCH2OTSR` reader"]
pub type R = crate::R<VBTWCH2OTSR_SPEC>;
#[doc = "Register `VBTWCH2OTSR` writer"]
pub type W = crate::W<VBTWCH2OTSR_SPEC>;
#[doc = "Field `CH2VCH0TE` reader - VBATWIO2 Output VBATWIO0 Trigger Enable"]
pub type CH2VCH0TE_R = crate::BitReader<CH2VCH0TE_A>;
#[doc = "VBATWIO2 Output VBATWIO0 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2VCH0TE_A {
    #[doc = "0: VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is enabled."]
    _1 = 1,
}
impl From<CH2VCH0TE_A> for bool {
    #[inline(always)]
    fn from(variant: CH2VCH0TE_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2VCH0TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH2VCH0TE_A {
        match self.bits {
            false => CH2VCH0TE_A::_0,
            true => CH2VCH0TE_A::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2VCH0TE_A::_0
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2VCH0TE_A::_1
    }
}
#[doc = "Field `CH2VCH0TE` writer - VBATWIO2 Output VBATWIO0 Trigger Enable"]
pub type CH2VCH0TE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH2VCH0TE_A>;
impl<'a, REG, const O: u8> CH2VCH0TE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VCH0TE_A::_0)
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VCH0TE_A::_1)
    }
}
#[doc = "Field `CH2VCH1TE` reader - VBATWIO2 Output VBATWIO1 Trigger Enable"]
pub type CH2VCH1TE_R = crate::BitReader<CH2VCH1TE_A>;
#[doc = "VBATWIO2 Output VBATWIO1 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2VCH1TE_A {
    #[doc = "0: VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is enabled."]
    _1 = 1,
}
impl From<CH2VCH1TE_A> for bool {
    #[inline(always)]
    fn from(variant: CH2VCH1TE_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2VCH1TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH2VCH1TE_A {
        match self.bits {
            false => CH2VCH1TE_A::_0,
            true => CH2VCH1TE_A::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2VCH1TE_A::_0
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2VCH1TE_A::_1
    }
}
#[doc = "Field `CH2VCH1TE` writer - VBATWIO2 Output VBATWIO1 Trigger Enable"]
pub type CH2VCH1TE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH2VCH1TE_A>;
impl<'a, REG, const O: u8> CH2VCH1TE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VCH1TE_A::_0)
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VCH1TE_A::_1)
    }
}
#[doc = "Field `CH2VRTCTE` reader - VBATWIO2 Output RTC Periodic Signal Enable"]
pub type CH2VRTCTE_R = crate::BitReader<CH2VRTCTE_A>;
#[doc = "VBATWIO2 Output RTC Periodic Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2VRTCTE_A {
    #[doc = "0: VBATT wakeup I/O 2 output trigger by the RTC periodic signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 2 output trigger by the RTC periodic signal is enabled."]
    _1 = 1,
}
impl From<CH2VRTCTE_A> for bool {
    #[inline(always)]
    fn from(variant: CH2VRTCTE_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2VRTCTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH2VRTCTE_A {
        match self.bits {
            false => CH2VRTCTE_A::_0,
            true => CH2VRTCTE_A::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2VRTCTE_A::_0
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC periodic signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2VRTCTE_A::_1
    }
}
#[doc = "Field `CH2VRTCTE` writer - VBATWIO2 Output RTC Periodic Signal Enable"]
pub type CH2VRTCTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH2VRTCTE_A>;
impl<'a, REG, const O: u8> CH2VRTCTE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VRTCTE_A::_0)
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC periodic signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VRTCTE_A::_1)
    }
}
#[doc = "Field `CH2VRTCATE` reader - VBATWIO2 Output RTC Alarm Signal Enable"]
pub type CH2VRTCATE_R = crate::BitReader<CH2VRTCATE_A>;
#[doc = "VBATWIO2 Output RTC Alarm Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2VRTCATE_A {
    #[doc = "0: VBATT wakeup I/O 2 output trigger by the RTC alarm signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 2 output trigger by the RTC alarm signal is enabled."]
    _1 = 1,
}
impl From<CH2VRTCATE_A> for bool {
    #[inline(always)]
    fn from(variant: CH2VRTCATE_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2VRTCATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH2VRTCATE_A {
        match self.bits {
            false => CH2VRTCATE_A::_0,
            true => CH2VRTCATE_A::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2VRTCATE_A::_0
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2VRTCATE_A::_1
    }
}
#[doc = "Field `CH2VRTCATE` writer - VBATWIO2 Output RTC Alarm Signal Enable"]
pub type CH2VRTCATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH2VRTCATE_A>;
impl<'a, REG, const O: u8> CH2VRTCATE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VRTCATE_A::_0)
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CH2VRTCATE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATWIO2 Output VBATWIO0 Trigger Enable"]
    #[inline(always)]
    pub fn ch2vch0te(&self) -> CH2VCH0TE_R {
        CH2VCH0TE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATWIO2 Output VBATWIO1 Trigger Enable"]
    #[inline(always)]
    pub fn ch2vch1te(&self) -> CH2VCH1TE_R {
        CH2VCH1TE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - VBATWIO2 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn ch2vrtcte(&self) -> CH2VRTCTE_R {
        CH2VRTCTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATWIO2 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn ch2vrtcate(&self) -> CH2VRTCATE_R {
        CH2VRTCATE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBATWIO2 Output VBATWIO0 Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2vch0te(&mut self) -> CH2VCH0TE_W<VBTWCH2OTSR_SPEC, 0> {
        CH2VCH0TE_W::new(self)
    }
    #[doc = "Bit 1 - VBATWIO2 Output VBATWIO1 Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2vch1te(&mut self) -> CH2VCH1TE_W<VBTWCH2OTSR_SPEC, 1> {
        CH2VCH1TE_W::new(self)
    }
    #[doc = "Bit 3 - VBATWIO2 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2vrtcte(&mut self) -> CH2VRTCTE_W<VBTWCH2OTSR_SPEC, 3> {
        CH2VRTCTE_W::new(self)
    }
    #[doc = "Bit 4 - VBATWIO2 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2vrtcate(&mut self) -> CH2VRTCATE_W<VBTWCH2OTSR_SPEC, 4> {
        CH2VRTCATE_W::new(self)
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
#[doc = "VBATT Wakeup I/O 2 Output Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbtwch2otsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbtwch2otsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VBTWCH2OTSR_SPEC;
impl crate::RegisterSpec for VBTWCH2OTSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtwch2otsr::R`](R) reader structure"]
impl crate::Readable for VBTWCH2OTSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vbtwch2otsr::W`](W) writer structure"]
impl crate::Writable for VBTWCH2OTSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VBTWCH2OTSR to value 0"]
impl crate::Resettable for VBTWCH2OTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
