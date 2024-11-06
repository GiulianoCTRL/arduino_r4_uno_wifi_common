#[doc = "Register `NMIER` reader"]
pub type R = crate::R<NMIER_SPEC>;
#[doc = "Register `NMIER` writer"]
pub type W = crate::W<NMIER_SPEC>;
#[doc = "Field `IWDTEN` reader - IWDT Underflow/Refresh Error Interrupt Enable"]
pub type IWDTEN_R = crate::BitReader<IWDTEN_A>;
#[doc = "IWDT Underflow/Refresh Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<IWDTEN_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IWDTEN_A {
        match self.bits {
            false => IWDTEN_A::_0,
            true => IWDTEN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IWDTEN_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IWDTEN_A::_1
    }
}
#[doc = "Field `IWDTEN` writer - IWDT Underflow/Refresh Error Interrupt Enable"]
pub type IWDTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IWDTEN_A>;
impl<'a, REG, const O: u8> IWDTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IWDTEN_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IWDTEN_A::_1)
    }
}
#[doc = "Field `WDTEN` reader - WDT Underflow/Refresh Error Interrupt Enable"]
pub type WDTEN_R = crate::BitReader<WDTEN_A>;
#[doc = "WDT Underflow/Refresh Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<WDTEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDTEN_A {
        match self.bits {
            false => WDTEN_A::_0,
            true => WDTEN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDTEN_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDTEN_A::_1
    }
}
#[doc = "Field `WDTEN` writer - WDT Underflow/Refresh Error Interrupt Enable"]
pub type WDTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDTEN_A>;
impl<'a, REG, const O: u8> WDTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WDTEN_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTEN_A::_1)
    }
}
#[doc = "Field `LVD1EN` reader - Voltage-Monitoring 1 Interrupt Enable"]
pub type LVD1EN_R = crate::BitReader<LVD1EN_A>;
#[doc = "Voltage-Monitoring 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1EN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<LVD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVD1EN_A {
        match self.bits {
            false => LVD1EN_A::_0,
            true => LVD1EN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1EN_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1EN_A::_1
    }
}
#[doc = "Field `LVD1EN` writer - Voltage-Monitoring 1 Interrupt Enable"]
pub type LVD1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LVD1EN_A>;
impl<'a, REG, const O: u8> LVD1EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1EN_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1EN_A::_1)
    }
}
#[doc = "Field `LVD2EN` reader - Voltage-Monitoring 2 Interrupt Enable"]
pub type LVD2EN_R = crate::BitReader<LVD2EN_A>;
#[doc = "Voltage-Monitoring 2 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2EN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<LVD2EN_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVD2EN_A {
        match self.bits {
            false => LVD2EN_A::_0,
            true => LVD2EN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2EN_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2EN_A::_1
    }
}
#[doc = "Field `LVD2EN` writer - Voltage-Monitoring 2 Interrupt Enable"]
pub type LVD2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LVD2EN_A>;
impl<'a, REG, const O: u8> LVD2EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2EN_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2EN_A::_1)
    }
}
#[doc = "Field `VBATTEN` reader - VBATT monitor Interrupt Enable"]
pub type VBATTEN_R = crate::BitReader<VBATTEN_A>;
#[doc = "VBATT monitor Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATTEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<VBATTEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBATTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VBATTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATTEN_A {
        match self.bits {
            false => VBATTEN_A::_0,
            true => VBATTEN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBATTEN_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBATTEN_A::_1
    }
}
#[doc = "Field `VBATTEN` writer - VBATT monitor Interrupt Enable"]
pub type VBATTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VBATTEN_A>;
impl<'a, REG, const O: u8> VBATTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBATTEN_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBATTEN_A::_1)
    }
}
#[doc = "Field `OSTEN` reader - Oscillation Stop Detection Interrupt Enable"]
pub type OSTEN_R = crate::BitReader<OSTEN_A>;
#[doc = "Oscillation Stop Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<OSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: OSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSTEN_A {
        match self.bits {
            false => OSTEN_A::_0,
            true => OSTEN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTEN_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTEN_A::_1
    }
}
#[doc = "Field `OSTEN` writer - Oscillation Stop Detection Interrupt Enable"]
pub type OSTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSTEN_A>;
impl<'a, REG, const O: u8> OSTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTEN_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTEN_A::_1)
    }
}
#[doc = "Field `NMIEN` reader - NMI Pin Interrupt Enable"]
pub type NMIEN_R = crate::BitReader<NMIEN_A>;
#[doc = "NMI Pin Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMIEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<NMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: NMIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NMIEN_A {
        match self.bits {
            false => NMIEN_A::_0,
            true => NMIEN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NMIEN_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NMIEN_A::_1
    }
}
#[doc = "Field `NMIEN` writer - NMI Pin Interrupt Enable"]
pub type NMIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NMIEN_A>;
impl<'a, REG, const O: u8> NMIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NMIEN_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NMIEN_A::_1)
    }
}
#[doc = "Field `RPEEN` reader - RAM Parity Error Interrupt Enable"]
pub type RPEEN_R = crate::BitReader<RPEEN_A>;
#[doc = "RAM Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPEEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<RPEEN_A> for bool {
    #[inline(always)]
    fn from(variant: RPEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RPEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPEEN_A {
        match self.bits {
            false => RPEEN_A::_0,
            true => RPEEN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPEEN_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPEEN_A::_1
    }
}
#[doc = "Field `RPEEN` writer - RAM Parity Error Interrupt Enable"]
pub type RPEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RPEEN_A>;
impl<'a, REG, const O: u8> RPEEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPEEN_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPEEN_A::_1)
    }
}
#[doc = "Field `RECCEN` reader - RAM ECC Error Interrupt Enable"]
pub type RECCEN_R = crate::BitReader<RECCEN_A>;
#[doc = "RAM ECC Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECCEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<RECCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RECCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RECCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RECCEN_A {
        match self.bits {
            false => RECCEN_A::_0,
            true => RECCEN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECCEN_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECCEN_A::_1
    }
}
#[doc = "Field `RECCEN` writer - RAM ECC Error Interrupt Enable"]
pub type RECCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RECCEN_A>;
impl<'a, REG, const O: u8> RECCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RECCEN_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RECCEN_A::_1)
    }
}
#[doc = "Field `BUSSEN` reader - MPU Bus Slave Error Interrupt Enable"]
pub type BUSSEN_R = crate::BitReader<BUSSEN_A>;
#[doc = "MPU Bus Slave Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSSEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<BUSSEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUSSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSSEN_A {
        match self.bits {
            false => BUSSEN_A::_0,
            true => BUSSEN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSSEN_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSSEN_A::_1
    }
}
#[doc = "Field `BUSSEN` writer - MPU Bus Slave Error Interrupt Enable"]
pub type BUSSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BUSSEN_A>;
impl<'a, REG, const O: u8> BUSSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BUSSEN_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BUSSEN_A::_1)
    }
}
#[doc = "Field `BUSMEN` reader - MPU Bus Master Error Interrupt Enable"]
pub type BUSMEN_R = crate::BitReader<BUSMEN_A>;
#[doc = "MPU Bus Master Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSMEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<BUSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSMEN_A {
        match self.bits {
            false => BUSMEN_A::_0,
            true => BUSMEN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSMEN_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSMEN_A::_1
    }
}
#[doc = "Field `BUSMEN` writer - MPU Bus Master Error Interrupt Enable"]
pub type BUSMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BUSMEN_A>;
impl<'a, REG, const O: u8> BUSMEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BUSMEN_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BUSMEN_A::_1)
    }
}
#[doc = "Field `SPEEN` reader - CPU Stack pointer monitor Interrupt Enable"]
pub type SPEEN_R = crate::BitReader<SPEEN_A>;
#[doc = "CPU Stack pointer monitor Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPEEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<SPEEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPEEN_A {
        match self.bits {
            false => SPEEN_A::_0,
            true => SPEEN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPEEN_A::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPEEN_A::_1
    }
}
#[doc = "Field `SPEEN` writer - CPU Stack pointer monitor Interrupt Enable"]
pub type SPEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPEEN_A>;
impl<'a, REG, const O: u8> SPEEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPEEN_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPEEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IWDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub fn iwdten(&self) -> IWDTEN_R {
        IWDTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage-Monitoring 1 Interrupt Enable"]
    #[inline(always)]
    pub fn lvd1en(&self) -> LVD1EN_R {
        LVD1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage-Monitoring 2 Interrupt Enable"]
    #[inline(always)]
    pub fn lvd2en(&self) -> LVD2EN_R {
        LVD2EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATT monitor Interrupt Enable"]
    #[inline(always)]
    pub fn vbatten(&self) -> VBATTEN_R {
        VBATTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn osten(&self) -> OSTEN_R {
        OSTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NMI Pin Interrupt Enable"]
    #[inline(always)]
    pub fn nmien(&self) -> NMIEN_R {
        NMIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RAM Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn rpeen(&self) -> RPEEN_R {
        RPEEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RAM ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn reccen(&self) -> RECCEN_R {
        RECCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MPU Bus Slave Error Interrupt Enable"]
    #[inline(always)]
    pub fn bussen(&self) -> BUSSEN_R {
        BUSSEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MPU Bus Master Error Interrupt Enable"]
    #[inline(always)]
    pub fn busmen(&self) -> BUSMEN_R {
        BUSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU Stack pointer monitor Interrupt Enable"]
    #[inline(always)]
    pub fn speen(&self) -> SPEEN_R {
        SPEEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IWDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iwdten(&mut self) -> IWDTEN_W<NMIER_SPEC, 0> {
        IWDTEN_W::new(self)
    }
    #[doc = "Bit 1 - WDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdten(&mut self) -> WDTEN_W<NMIER_SPEC, 1> {
        WDTEN_W::new(self)
    }
    #[doc = "Bit 2 - Voltage-Monitoring 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvd1en(&mut self) -> LVD1EN_W<NMIER_SPEC, 2> {
        LVD1EN_W::new(self)
    }
    #[doc = "Bit 3 - Voltage-Monitoring 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvd2en(&mut self) -> LVD2EN_W<NMIER_SPEC, 3> {
        LVD2EN_W::new(self)
    }
    #[doc = "Bit 4 - VBATT monitor Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbatten(&mut self) -> VBATTEN_W<NMIER_SPEC, 4> {
        VBATTEN_W::new(self)
    }
    #[doc = "Bit 6 - Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osten(&mut self) -> OSTEN_W<NMIER_SPEC, 6> {
        OSTEN_W::new(self)
    }
    #[doc = "Bit 7 - NMI Pin Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nmien(&mut self) -> NMIEN_W<NMIER_SPEC, 7> {
        NMIEN_W::new(self)
    }
    #[doc = "Bit 8 - RAM Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rpeen(&mut self) -> RPEEN_W<NMIER_SPEC, 8> {
        RPEEN_W::new(self)
    }
    #[doc = "Bit 9 - RAM ECC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reccen(&mut self) -> RECCEN_W<NMIER_SPEC, 9> {
        RECCEN_W::new(self)
    }
    #[doc = "Bit 10 - MPU Bus Slave Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bussen(&mut self) -> BUSSEN_W<NMIER_SPEC, 10> {
        BUSSEN_W::new(self)
    }
    #[doc = "Bit 11 - MPU Bus Master Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn busmen(&mut self) -> BUSMEN_W<NMIER_SPEC, 11> {
        BUSMEN_W::new(self)
    }
    #[doc = "Bit 12 - CPU Stack pointer monitor Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn speen(&mut self) -> SPEEN_W<NMIER_SPEC, 12> {
        SPEEN_W::new(self)
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
#[doc = "Non-Maskable Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NMIER_SPEC;
impl crate::RegisterSpec for NMIER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nmier::R`](R) reader structure"]
impl crate::Readable for NMIER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nmier::W`](W) writer structure"]
impl crate::Writable for NMIER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NMIER to value 0"]
impl crate::Resettable for NMIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
