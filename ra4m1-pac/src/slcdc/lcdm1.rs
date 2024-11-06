#[doc = "Register `LCDM1` reader"]
pub type R = crate::R<LCDM1_SPEC>;
#[doc = "Register `LCDM1` writer"]
pub type W = crate::W<LCDM1_SPEC>;
#[doc = "Field `LCDVLM` reader - Voltage Boosting Pin Initial Value Switching Control"]
pub type LCDVLM_R = crate::BitReader<LCDVLM_A>;
#[doc = "Voltage Boosting Pin Initial Value Switching Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDVLM_A {
    #[doc = "0: Set when VDD >= 2.7 V"]
    _0 = 0,
    #[doc = "1: Set when VDD &lt;= 4.2 V"]
    _1 = 1,
}
impl From<LCDVLM_A> for bool {
    #[inline(always)]
    fn from(variant: LCDVLM_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDVLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCDVLM_A {
        match self.bits {
            false => LCDVLM_A::_0,
            true => LCDVLM_A::_1,
        }
    }
    #[doc = "Set when VDD >= 2.7 V"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCDVLM_A::_0
    }
    #[doc = "Set when VDD &lt;= 4.2 V"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCDVLM_A::_1
    }
}
#[doc = "Field `LCDVLM` writer - Voltage Boosting Pin Initial Value Switching Control"]
pub type LCDVLM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LCDVLM_A>;
impl<'a, REG, const O: u8> LCDVLM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set when VDD >= 2.7 V"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LCDVLM_A::_0)
    }
    #[doc = "Set when VDD &lt;= 4.2 V"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LCDVLM_A::_1)
    }
}
#[doc = "Field `LCDSEL` reader - Display data area control"]
pub type LCDSEL_R = crate::BitReader<LCDSEL_A>;
#[doc = "Display data area control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDSEL_A {
    #[doc = "0: Displaying an A-pattern area data (lower four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    _0 = 0,
    #[doc = "1: Displaying a B-pattern area data (higher four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    _1 = 1,
}
impl From<LCDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: LCDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCDSEL_A {
        match self.bits {
            false => LCDSEL_A::_0,
            true => LCDSEL_A::_1,
        }
    }
    #[doc = "Displaying an A-pattern area data (lower four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCDSEL_A::_0
    }
    #[doc = "Displaying a B-pattern area data (higher four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCDSEL_A::_1
    }
}
#[doc = "Field `LCDSEL` writer - Display data area control"]
pub type LCDSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LCDSEL_A>;
impl<'a, REG, const O: u8> LCDSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Displaying an A-pattern area data (lower four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSEL_A::_0)
    }
    #[doc = "Displaying a B-pattern area data (higher four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LCDSEL_A::_1)
    }
}
#[doc = "Field `BLON` reader - Display data area control"]
pub type BLON_R = crate::BitReader<BLON_A>;
#[doc = "Display data area control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLON_A {
    #[doc = "0: Displaying an A-pattern area data (lower four bits of LCD display data register)(LCDSEL=0)/Displaying a B-pattern area data (higher four bits of LCD display data register)(LCDSEL=1)"]
    _0 = 0,
    #[doc = "1: Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))"]
    _1 = 1,
}
impl From<BLON_A> for bool {
    #[inline(always)]
    fn from(variant: BLON_A) -> Self {
        variant as u8 != 0
    }
}
impl BLON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLON_A {
        match self.bits {
            false => BLON_A::_0,
            true => BLON_A::_1,
        }
    }
    #[doc = "Displaying an A-pattern area data (lower four bits of LCD display data register)(LCDSEL=0)/Displaying a B-pattern area data (higher four bits of LCD display data register)(LCDSEL=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLON_A::_0
    }
    #[doc = "Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLON_A::_1
    }
}
#[doc = "Field `BLON` writer - Display data area control"]
pub type BLON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BLON_A>;
impl<'a, REG, const O: u8> BLON_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Displaying an A-pattern area data (lower four bits of LCD display data register)(LCDSEL=0)/Displaying a B-pattern area data (higher four bits of LCD display data register)(LCDSEL=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BLON_A::_0)
    }
    #[doc = "Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BLON_A::_1)
    }
}
#[doc = "Field `VLCON` reader - Voltage boost circuit or capacitor split circuit operation enable/disable"]
pub type VLCON_R = crate::BitReader<VLCON_A>;
#[doc = "Voltage boost circuit or capacitor split circuit operation enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VLCON_A {
    #[doc = "0: Stops voltage boost circuit or capacitor split circuit operation"]
    _0 = 0,
    #[doc = "1: Enables voltage boost circuit or capacitor split circuit operation"]
    _1 = 1,
}
impl From<VLCON_A> for bool {
    #[inline(always)]
    fn from(variant: VLCON_A) -> Self {
        variant as u8 != 0
    }
}
impl VLCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VLCON_A {
        match self.bits {
            false => VLCON_A::_0,
            true => VLCON_A::_1,
        }
    }
    #[doc = "Stops voltage boost circuit or capacitor split circuit operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VLCON_A::_0
    }
    #[doc = "Enables voltage boost circuit or capacitor split circuit operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VLCON_A::_1
    }
}
#[doc = "Field `VLCON` writer - Voltage boost circuit or capacitor split circuit operation enable/disable"]
pub type VLCON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VLCON_A>;
impl<'a, REG, const O: u8> VLCON_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stops voltage boost circuit or capacitor split circuit operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VLCON_A::_0)
    }
    #[doc = "Enables voltage boost circuit or capacitor split circuit operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VLCON_A::_1)
    }
}
#[doc = "Field `SCOC` reader - LCD Display Enable/Disable"]
pub type SCOC_R = crate::BitReader<SCOC_A>;
#[doc = "LCD Display Enable/Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCOC_A {
    #[doc = "0: Output ground level to segment/common pin(LCDON=0)/Output ground level to segment/common pin(LCDON=1)"]
    _0 = 0,
    #[doc = "1: Display off (all segment outputs are deselected)(LCDON=0)/Display on(LCDON=1)"]
    _1 = 1,
}
impl From<SCOC_A> for bool {
    #[inline(always)]
    fn from(variant: SCOC_A) -> Self {
        variant as u8 != 0
    }
}
impl SCOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCOC_A {
        match self.bits {
            false => SCOC_A::_0,
            true => SCOC_A::_1,
        }
    }
    #[doc = "Output ground level to segment/common pin(LCDON=0)/Output ground level to segment/common pin(LCDON=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCOC_A::_0
    }
    #[doc = "Display off (all segment outputs are deselected)(LCDON=0)/Display on(LCDON=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCOC_A::_1
    }
}
#[doc = "Field `SCOC` writer - LCD Display Enable/Disable"]
pub type SCOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SCOC_A>;
impl<'a, REG, const O: u8> SCOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output ground level to segment/common pin(LCDON=0)/Output ground level to segment/common pin(LCDON=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SCOC_A::_0)
    }
    #[doc = "Display off (all segment outputs are deselected)(LCDON=0)/Display on(LCDON=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SCOC_A::_1)
    }
}
#[doc = "Field `LCDON` reader - LCD Display Enable/Disable"]
pub type LCDON_R = crate::BitReader<LCDON_A>;
#[doc = "LCD Display Enable/Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDON_A {
    #[doc = "0: Output ground level to segment/common pin(SCOC=0)/Display off (all segment outputs are deselected)(SCOC=1)"]
    _0 = 0,
    #[doc = "1: Output ground level to segment/common pin(SCOC=0)/Display on(SCOC=1)"]
    _1 = 1,
}
impl From<LCDON_A> for bool {
    #[inline(always)]
    fn from(variant: LCDON_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCDON_A {
        match self.bits {
            false => LCDON_A::_0,
            true => LCDON_A::_1,
        }
    }
    #[doc = "Output ground level to segment/common pin(SCOC=0)/Display off (all segment outputs are deselected)(SCOC=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCDON_A::_0
    }
    #[doc = "Output ground level to segment/common pin(SCOC=0)/Display on(SCOC=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCDON_A::_1
    }
}
#[doc = "Field `LCDON` writer - LCD Display Enable/Disable"]
pub type LCDON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LCDON_A>;
impl<'a, REG, const O: u8> LCDON_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output ground level to segment/common pin(SCOC=0)/Display off (all segment outputs are deselected)(SCOC=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LCDON_A::_0)
    }
    #[doc = "Output ground level to segment/common pin(SCOC=0)/Display on(SCOC=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LCDON_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Boosting Pin Initial Value Switching Control"]
    #[inline(always)]
    pub fn lcdvlm(&self) -> LCDVLM_R {
        LCDVLM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Display data area control"]
    #[inline(always)]
    pub fn lcdsel(&self) -> LCDSEL_R {
        LCDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Display data area control"]
    #[inline(always)]
    pub fn blon(&self) -> BLON_R {
        BLON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Voltage boost circuit or capacitor split circuit operation enable/disable"]
    #[inline(always)]
    pub fn vlcon(&self) -> VLCON_R {
        VLCON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD Display Enable/Disable"]
    #[inline(always)]
    pub fn scoc(&self) -> SCOC_R {
        SCOC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD Display Enable/Disable"]
    #[inline(always)]
    pub fn lcdon(&self) -> LCDON_R {
        LCDON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Boosting Pin Initial Value Switching Control"]
    #[inline(always)]
    #[must_use]
    pub fn lcdvlm(&mut self) -> LCDVLM_W<LCDM1_SPEC, 0> {
        LCDVLM_W::new(self)
    }
    #[doc = "Bit 3 - Display data area control"]
    #[inline(always)]
    #[must_use]
    pub fn lcdsel(&mut self) -> LCDSEL_W<LCDM1_SPEC, 3> {
        LCDSEL_W::new(self)
    }
    #[doc = "Bit 4 - Display data area control"]
    #[inline(always)]
    #[must_use]
    pub fn blon(&mut self) -> BLON_W<LCDM1_SPEC, 4> {
        BLON_W::new(self)
    }
    #[doc = "Bit 5 - Voltage boost circuit or capacitor split circuit operation enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn vlcon(&mut self) -> VLCON_W<LCDM1_SPEC, 5> {
        VLCON_W::new(self)
    }
    #[doc = "Bit 6 - LCD Display Enable/Disable"]
    #[inline(always)]
    #[must_use]
    pub fn scoc(&mut self) -> SCOC_W<LCDM1_SPEC, 6> {
        SCOC_W::new(self)
    }
    #[doc = "Bit 7 - LCD Display Enable/Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lcdon(&mut self) -> LCDON_W<LCDM1_SPEC, 7> {
        LCDON_W::new(self)
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
#[doc = "LCD Mode Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcdm1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcdm1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCDM1_SPEC;
impl crate::RegisterSpec for LCDM1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm1::R`](R) reader structure"]
impl crate::Readable for LCDM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcdm1::W`](W) writer structure"]
impl crate::Writable for LCDM1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCDM1 to value 0"]
impl crate::Resettable for LCDM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
