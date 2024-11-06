#[doc = "Register `DVSTCTR0` reader"]
pub type R = crate::R<DVSTCTR0_SPEC>;
#[doc = "Register `DVSTCTR0` writer"]
pub type W = crate::W<DVSTCTR0_SPEC>;
#[doc = "Field `RHST` reader - USB Bus Reset Status"]
pub type RHST_R = crate::FieldReader<RHST_A>;
#[doc = "USB Bus Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RHST_A {
    #[doc = "0: Communication speed not determined"]
    _000 = 0,
    #[doc = "1: Low-speed connection(When the host controller is selected) /USB bus reset in progress( When the function controller is selected)"]
    _001 = 1,
    #[doc = "2: Full-speed connection(When the host controller is selected) /USB bus reset in progress or full-speed connection(When the function controller is selected)"]
    _010 = 2,
    #[doc = "3: Setting prohibited"]
    _011 = 3,
}
impl From<RHST_A> for u8 {
    #[inline(always)]
    fn from(variant: RHST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RHST_A {
    type Ux = u8;
}
impl RHST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RHST_A> {
        match self.bits {
            0 => Some(RHST_A::_000),
            1 => Some(RHST_A::_001),
            2 => Some(RHST_A::_010),
            3 => Some(RHST_A::_011),
            _ => None,
        }
    }
    #[doc = "Communication speed not determined"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RHST_A::_000
    }
    #[doc = "Low-speed connection(When the host controller is selected) /USB bus reset in progress( When the function controller is selected)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RHST_A::_001
    }
    #[doc = "Full-speed connection(When the host controller is selected) /USB bus reset in progress or full-speed connection(When the function controller is selected)"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RHST_A::_010
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RHST_A::_011
    }
}
#[doc = "Field `UACT` reader - USB Bus Enable"]
pub type UACT_R = crate::BitReader<UACT_A>;
#[doc = "USB Bus Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UACT_A {
    #[doc = "0: Downstream port is disabled (SOF transmission is disabled)."]
    _0 = 0,
    #[doc = "1: Downstream port is enabled (SOF transmission is enabled)."]
    _1 = 1,
}
impl From<UACT_A> for bool {
    #[inline(always)]
    fn from(variant: UACT_A) -> Self {
        variant as u8 != 0
    }
}
impl UACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UACT_A {
        match self.bits {
            false => UACT_A::_0,
            true => UACT_A::_1,
        }
    }
    #[doc = "Downstream port is disabled (SOF transmission is disabled)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UACT_A::_0
    }
    #[doc = "Downstream port is enabled (SOF transmission is enabled)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UACT_A::_1
    }
}
#[doc = "Field `UACT` writer - USB Bus Enable"]
pub type UACT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UACT_A>;
impl<'a, REG, const O: u8> UACT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Downstream port is disabled (SOF transmission is disabled)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UACT_A::_0)
    }
    #[doc = "Downstream port is enabled (SOF transmission is enabled)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UACT_A::_1)
    }
}
#[doc = "Field `RESUME` reader - Resume Output"]
pub type RESUME_R = crate::BitReader<RESUME_A>;
#[doc = "Resume Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESUME_A {
    #[doc = "0: Resume signal is not output."]
    _0 = 0,
    #[doc = "1: Resume signal is output."]
    _1 = 1,
}
impl From<RESUME_A> for bool {
    #[inline(always)]
    fn from(variant: RESUME_A) -> Self {
        variant as u8 != 0
    }
}
impl RESUME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESUME_A {
        match self.bits {
            false => RESUME_A::_0,
            true => RESUME_A::_1,
        }
    }
    #[doc = "Resume signal is not output."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESUME_A::_0
    }
    #[doc = "Resume signal is output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESUME_A::_1
    }
}
#[doc = "Field `RESUME` writer - Resume Output"]
pub type RESUME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESUME_A>;
impl<'a, REG, const O: u8> RESUME_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resume signal is not output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RESUME_A::_0)
    }
    #[doc = "Resume signal is output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RESUME_A::_1)
    }
}
#[doc = "Field `USBRST` reader - USB Bus Reset Output"]
pub type USBRST_R = crate::BitReader<USBRST_A>;
#[doc = "USB Bus Reset Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRST_A {
    #[doc = "0: USB bus reset signal is not output."]
    _0 = 0,
    #[doc = "1: USB bus reset signal is output."]
    _1 = 1,
}
impl From<USBRST_A> for bool {
    #[inline(always)]
    fn from(variant: USBRST_A) -> Self {
        variant as u8 != 0
    }
}
impl USBRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBRST_A {
        match self.bits {
            false => USBRST_A::_0,
            true => USBRST_A::_1,
        }
    }
    #[doc = "USB bus reset signal is not output."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBRST_A::_0
    }
    #[doc = "USB bus reset signal is output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBRST_A::_1
    }
}
#[doc = "Field `USBRST` writer - USB Bus Reset Output"]
pub type USBRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USBRST_A>;
impl<'a, REG, const O: u8> USBRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB bus reset signal is not output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USBRST_A::_0)
    }
    #[doc = "USB bus reset signal is output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USBRST_A::_1)
    }
}
#[doc = "Field `RWUPE` reader - Wakeup Detection Enable"]
pub type RWUPE_R = crate::BitReader<RWUPE_A>;
#[doc = "Wakeup Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWUPE_A {
    #[doc = "0: Downstream port wakeup is disabled."]
    _0 = 0,
    #[doc = "1: Downstream port wakeup is enabled."]
    _1 = 1,
}
impl From<RWUPE_A> for bool {
    #[inline(always)]
    fn from(variant: RWUPE_A) -> Self {
        variant as u8 != 0
    }
}
impl RWUPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RWUPE_A {
        match self.bits {
            false => RWUPE_A::_0,
            true => RWUPE_A::_1,
        }
    }
    #[doc = "Downstream port wakeup is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWUPE_A::_0
    }
    #[doc = "Downstream port wakeup is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWUPE_A::_1
    }
}
#[doc = "Field `RWUPE` writer - Wakeup Detection Enable"]
pub type RWUPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RWUPE_A>;
impl<'a, REG, const O: u8> RWUPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Downstream port wakeup is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RWUPE_A::_0)
    }
    #[doc = "Downstream port wakeup is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RWUPE_A::_1)
    }
}
#[doc = "Field `WKUP` reader - Wakeup Output"]
pub type WKUP_R = crate::BitReader<WKUP_A>;
#[doc = "Wakeup Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUP_A {
    #[doc = "0: Remote wakeup signal is not output."]
    _0 = 0,
    #[doc = "1: Remote wakeup signal is output."]
    _1 = 1,
}
impl From<WKUP_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WKUP_A {
        match self.bits {
            false => WKUP_A::_0,
            true => WKUP_A::_1,
        }
    }
    #[doc = "Remote wakeup signal is not output."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WKUP_A::_0
    }
    #[doc = "Remote wakeup signal is output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WKUP_A::_1
    }
}
#[doc = "Field `WKUP` writer - Wakeup Output"]
pub type WKUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WKUP_A>;
impl<'a, REG, const O: u8> WKUP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Remote wakeup signal is not output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WKUP_A::_0)
    }
    #[doc = "Remote wakeup signal is output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WKUP_A::_1)
    }
}
#[doc = "Field `VBUSEN` reader - USB_VBUSEN Output Pin Control"]
pub type VBUSEN_R = crate::BitReader<VBUSEN_A>;
#[doc = "USB_VBUSEN Output Pin Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBUSEN_A {
    #[doc = "0: External USB_VBUSEN pin outputs low"]
    _0 = 0,
    #[doc = "1: External USB_VBUSEN pin outputs high"]
    _1 = 1,
}
impl From<VBUSEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VBUSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBUSEN_A {
        match self.bits {
            false => VBUSEN_A::_0,
            true => VBUSEN_A::_1,
        }
    }
    #[doc = "External USB_VBUSEN pin outputs low"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBUSEN_A::_0
    }
    #[doc = "External USB_VBUSEN pin outputs high"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBUSEN_A::_1
    }
}
#[doc = "Field `VBUSEN` writer - USB_VBUSEN Output Pin Control"]
pub type VBUSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VBUSEN_A>;
impl<'a, REG, const O: u8> VBUSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External USB_VBUSEN pin outputs low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBUSEN_A::_0)
    }
    #[doc = "External USB_VBUSEN pin outputs high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBUSEN_A::_1)
    }
}
#[doc = "Field `EXICEN` reader - USB_EXICEN Output Pin Control"]
pub type EXICEN_R = crate::BitReader<EXICEN_A>;
#[doc = "USB_EXICEN Output Pin Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXICEN_A {
    #[doc = "0: External USB_EXICEN pin outputs low"]
    _0 = 0,
    #[doc = "1: External USB_EXICEN pin outputs high"]
    _1 = 1,
}
impl From<EXICEN_A> for bool {
    #[inline(always)]
    fn from(variant: EXICEN_A) -> Self {
        variant as u8 != 0
    }
}
impl EXICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXICEN_A {
        match self.bits {
            false => EXICEN_A::_0,
            true => EXICEN_A::_1,
        }
    }
    #[doc = "External USB_EXICEN pin outputs low"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXICEN_A::_0
    }
    #[doc = "External USB_EXICEN pin outputs high"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXICEN_A::_1
    }
}
#[doc = "Field `EXICEN` writer - USB_EXICEN Output Pin Control"]
pub type EXICEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EXICEN_A>;
impl<'a, REG, const O: u8> EXICEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External USB_EXICEN pin outputs low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EXICEN_A::_0)
    }
    #[doc = "External USB_EXICEN pin outputs high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EXICEN_A::_1)
    }
}
#[doc = "Field `HNPBTOA` reader - Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
pub type HNPBTOA_R = crate::BitReader<HNPBTOA_A>;
#[doc = "Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HNPBTOA_A {
    #[doc = "0: Normal Operation"]
    _0 = 0,
    #[doc = "1: Switching from device B to device A is enabled"]
    _1 = 1,
}
impl From<HNPBTOA_A> for bool {
    #[inline(always)]
    fn from(variant: HNPBTOA_A) -> Self {
        variant as u8 != 0
    }
}
impl HNPBTOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HNPBTOA_A {
        match self.bits {
            false => HNPBTOA_A::_0,
            true => HNPBTOA_A::_1,
        }
    }
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HNPBTOA_A::_0
    }
    #[doc = "Switching from device B to device A is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HNPBTOA_A::_1
    }
}
#[doc = "Field `HNPBTOA` writer - Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
pub type HNPBTOA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HNPBTOA_A>;
impl<'a, REG, const O: u8> HNPBTOA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HNPBTOA_A::_0)
    }
    #[doc = "Switching from device B to device A is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HNPBTOA_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - USB Bus Reset Status"]
    #[inline(always)]
    pub fn rhst(&self) -> RHST_R {
        RHST_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - USB Bus Enable"]
    #[inline(always)]
    pub fn uact(&self) -> UACT_R {
        UACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Resume Output"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB Bus Reset Output"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Detection Enable"]
    #[inline(always)]
    pub fn rwupe(&self) -> RWUPE_R {
        RWUPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wakeup Output"]
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB_VBUSEN Output Pin Control"]
    #[inline(always)]
    pub fn vbusen(&self) -> VBUSEN_R {
        VBUSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB_EXICEN Output Pin Control"]
    #[inline(always)]
    pub fn exicen(&self) -> EXICEN_R {
        EXICEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
    #[inline(always)]
    pub fn hnpbtoa(&self) -> HNPBTOA_R {
        HNPBTOA_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - USB Bus Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uact(&mut self) -> UACT_W<DVSTCTR0_SPEC, 4> {
        UACT_W::new(self)
    }
    #[doc = "Bit 5 - Resume Output"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<DVSTCTR0_SPEC, 5> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 6 - USB Bus Reset Output"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<DVSTCTR0_SPEC, 6> {
        USBRST_W::new(self)
    }
    #[doc = "Bit 7 - Wakeup Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwupe(&mut self) -> RWUPE_W<DVSTCTR0_SPEC, 7> {
        RWUPE_W::new(self)
    }
    #[doc = "Bit 8 - Wakeup Output"]
    #[inline(always)]
    #[must_use]
    pub fn wkup(&mut self) -> WKUP_W<DVSTCTR0_SPEC, 8> {
        WKUP_W::new(self)
    }
    #[doc = "Bit 9 - USB_VBUSEN Output Pin Control"]
    #[inline(always)]
    #[must_use]
    pub fn vbusen(&mut self) -> VBUSEN_W<DVSTCTR0_SPEC, 9> {
        VBUSEN_W::new(self)
    }
    #[doc = "Bit 10 - USB_EXICEN Output Pin Control"]
    #[inline(always)]
    #[must_use]
    pub fn exicen(&mut self) -> EXICEN_W<DVSTCTR0_SPEC, 10> {
        EXICEN_W::new(self)
    }
    #[doc = "Bit 11 - Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
    #[inline(always)]
    #[must_use]
    pub fn hnpbtoa(&mut self) -> HNPBTOA_W<DVSTCTR0_SPEC, 11> {
        HNPBTOA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device State Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvstctr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvstctr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DVSTCTR0_SPEC;
impl crate::RegisterSpec for DVSTCTR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dvstctr0::R`](R) reader structure"]
impl crate::Readable for DVSTCTR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dvstctr0::W`](W) writer structure"]
impl crate::Writable for DVSTCTR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DVSTCTR0 to value 0"]
impl crate::Resettable for DVSTCTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
