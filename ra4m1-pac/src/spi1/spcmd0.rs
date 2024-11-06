#[doc = "Register `SPCMD0` reader"]
pub type R = crate::R<SPCMD0_SPEC>;
#[doc = "Register `SPCMD0` writer"]
pub type W = crate::W<SPCMD0_SPEC>;
#[doc = "Field `CPHA` reader - RSPCK Phase Setting"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "RSPCK Phase Setting\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: Data sampling on odd edge, data variation on even edge"]
    _0 = 0,
    #[doc = "1: Data variation on odd edge, data sampling on even edge"]
    _1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::_0,
            true => CPHA_A::_1,
        }
    }
    #[doc = "Data sampling on odd edge, data variation on even edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPHA_A::_0
    }
    #[doc = "Data variation on odd edge, data sampling on even edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPHA_A::_1
    }
}
#[doc = "Field `CPHA` writer - RSPCK Phase Setting"]
pub type CPHA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CPHA_A>;
impl<'a, REG, const O: u8> CPHA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data sampling on odd edge, data variation on even edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::_0)
    }
    #[doc = "Data variation on odd edge, data sampling on even edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::_1)
    }
}
#[doc = "Field `CPOL` reader - RSPCK Polarity Setting"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "RSPCK Polarity Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: RSPCK is low when idle"]
    _0 = 0,
    #[doc = "1: RSPCK is high when idle"]
    _1 = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::_0,
            true => CPOL_A::_1,
        }
    }
    #[doc = "RSPCK is low when idle"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOL_A::_0
    }
    #[doc = "RSPCK is high when idle"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOL_A::_1
    }
}
#[doc = "Field `CPOL` writer - RSPCK Polarity Setting"]
pub type CPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CPOL_A>;
impl<'a, REG, const O: u8> CPOL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RSPCK is low when idle"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::_0)
    }
    #[doc = "RSPCK is high when idle"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::_1)
    }
}
#[doc = "Field `BRDV` reader - Bit Rate Division Setting"]
pub type BRDV_R = crate::FieldReader<BRDV_A>;
#[doc = "Bit Rate Division Setting\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BRDV_A {
    #[doc = "0: These bits select the base bit rate"]
    _00 = 0,
    #[doc = "1: These bits select the base bit rate divided by 2"]
    _01 = 1,
    #[doc = "2: These bits select the base bit rate divided by 4"]
    _10 = 2,
    #[doc = "3: These bits select the base bit rate divided by 8"]
    _11 = 3,
}
impl From<BRDV_A> for u8 {
    #[inline(always)]
    fn from(variant: BRDV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BRDV_A {
    type Ux = u8;
}
impl BRDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRDV_A {
        match self.bits {
            0 => BRDV_A::_00,
            1 => BRDV_A::_01,
            2 => BRDV_A::_10,
            3 => BRDV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "These bits select the base bit rate"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BRDV_A::_00
    }
    #[doc = "These bits select the base bit rate divided by 2"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BRDV_A::_01
    }
    #[doc = "These bits select the base bit rate divided by 4"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BRDV_A::_10
    }
    #[doc = "These bits select the base bit rate divided by 8"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BRDV_A::_11
    }
}
#[doc = "Field `BRDV` writer - Bit Rate Division Setting"]
pub type BRDV_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, BRDV_A>;
impl<'a, REG, const O: u8> BRDV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "These bits select the base bit rate"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(BRDV_A::_00)
    }
    #[doc = "These bits select the base bit rate divided by 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(BRDV_A::_01)
    }
    #[doc = "These bits select the base bit rate divided by 4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(BRDV_A::_10)
    }
    #[doc = "These bits select the base bit rate divided by 8"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(BRDV_A::_11)
    }
}
#[doc = "Field `SSLA` reader - SSL Signal Assertion Setting"]
pub type SSLA_R = crate::FieldReader<SSLA_A>;
#[doc = "SSL Signal Assertion Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSLA_A {
    #[doc = "0: SSL0"]
    _000 = 0,
    #[doc = "1: SSL1"]
    _001 = 1,
    #[doc = "2: SSL2"]
    _010 = 2,
    #[doc = "3: SSL3"]
    _011 = 3,
}
impl From<SSLA_A> for u8 {
    #[inline(always)]
    fn from(variant: SSLA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SSLA_A {
    type Ux = u8;
}
impl SSLA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SSLA_A> {
        match self.bits {
            0 => Some(SSLA_A::_000),
            1 => Some(SSLA_A::_001),
            2 => Some(SSLA_A::_010),
            3 => Some(SSLA_A::_011),
            _ => None,
        }
    }
    #[doc = "SSL0"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SSLA_A::_000
    }
    #[doc = "SSL1"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SSLA_A::_001
    }
    #[doc = "SSL2"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SSLA_A::_010
    }
    #[doc = "SSL3"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SSLA_A::_011
    }
}
#[doc = "Field `SSLA` writer - SSL Signal Assertion Setting"]
pub type SSLA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SSLA_A>;
impl<'a, REG, const O: u8> SSLA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SSL0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SSLA_A::_000)
    }
    #[doc = "SSL1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SSLA_A::_001)
    }
    #[doc = "SSL2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SSLA_A::_010)
    }
    #[doc = "SSL3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SSLA_A::_011)
    }
}
#[doc = "Field `SPB` reader - RSPI Data Length Setting"]
pub type SPB_R = crate::FieldReader<SPB_A>;
#[doc = "RSPI Data Length Setting\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPB_A {
    #[doc = "0: 20 bits"]
    _0000 = 0,
    #[doc = "1: 24 bits"]
    _0001 = 1,
    #[doc = "2: 32 bits"]
    _0010 = 2,
    #[doc = "3: 32 bits"]
    _0011 = 3,
    #[doc = "8: 9 bits"]
    _1000 = 8,
    #[doc = "9: 10 bits"]
    _1001 = 9,
    #[doc = "10: 11 bits"]
    _1010 = 10,
    #[doc = "11: 12 bits"]
    _1011 = 11,
    #[doc = "12: 13 bits"]
    _1100 = 12,
    #[doc = "13: 14 bits"]
    _1101 = 13,
    #[doc = "14: 15 bits"]
    _1110 = 14,
    #[doc = "15: 16 bits"]
    _1111 = 15,
}
impl From<SPB_A> for u8 {
    #[inline(always)]
    fn from(variant: SPB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPB_A {
    type Ux = u8;
}
impl SPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPB_A> {
        match self.bits {
            0 => Some(SPB_A::_0000),
            1 => Some(SPB_A::_0001),
            2 => Some(SPB_A::_0010),
            3 => Some(SPB_A::_0011),
            8 => Some(SPB_A::_1000),
            9 => Some(SPB_A::_1001),
            10 => Some(SPB_A::_1010),
            11 => Some(SPB_A::_1011),
            12 => Some(SPB_A::_1100),
            13 => Some(SPB_A::_1101),
            14 => Some(SPB_A::_1110),
            15 => Some(SPB_A::_1111),
            _ => None,
        }
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SPB_A::_0000
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SPB_A::_0001
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SPB_A::_0010
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SPB_A::_0011
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == SPB_A::_1000
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == SPB_A::_1001
    }
    #[doc = "11 bits"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == SPB_A::_1010
    }
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == SPB_A::_1011
    }
    #[doc = "13 bits"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == SPB_A::_1100
    }
    #[doc = "14 bits"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == SPB_A::_1101
    }
    #[doc = "15 bits"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == SPB_A::_1110
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == SPB_A::_1111
    }
}
#[doc = "Field `SPB` writer - RSPI Data Length Setting"]
pub type SPB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, SPB_A>;
impl<'a, REG, const O: u8> SPB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_0000)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_0001)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_0010)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_0011)
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1000)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1001)
    }
    #[doc = "11 bits"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1010)
    }
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1011)
    }
    #[doc = "13 bits"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1100)
    }
    #[doc = "14 bits"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1101)
    }
    #[doc = "15 bits"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1110)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(SPB_A::_1111)
    }
}
#[doc = "Field `LSBF` reader - RSPI LSB First"]
pub type LSBF_R = crate::BitReader<LSBF_A>;
#[doc = "RSPI LSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBF_A {
    #[doc = "0: MSB first"]
    _0 = 0,
    #[doc = "1: LSB first"]
    _1 = 1,
}
impl From<LSBF_A> for bool {
    #[inline(always)]
    fn from(variant: LSBF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSBF_A {
        match self.bits {
            false => LSBF_A::_0,
            true => LSBF_A::_1,
        }
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LSBF_A::_0
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LSBF_A::_1
    }
}
#[doc = "Field `LSBF` writer - RSPI LSB First"]
pub type LSBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LSBF_A>;
impl<'a, REG, const O: u8> LSBF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LSBF_A::_0)
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LSBF_A::_1)
    }
}
#[doc = "Field `SPNDEN` reader - RSPI Next-Access Delay Enable"]
pub type SPNDEN_R = crate::BitReader<SPNDEN_A>;
#[doc = "RSPI Next-Access Delay Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPNDEN_A {
    #[doc = "0: A next-access delay of 1 RSPCK + 2 PCLK"]
    _0 = 0,
    #[doc = "1: A next-access delay is equal to the setting of the RSPI next-access delay register (SPND)"]
    _1 = 1,
}
impl From<SPNDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPNDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPNDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPNDEN_A {
        match self.bits {
            false => SPNDEN_A::_0,
            true => SPNDEN_A::_1,
        }
    }
    #[doc = "A next-access delay of 1 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPNDEN_A::_0
    }
    #[doc = "A next-access delay is equal to the setting of the RSPI next-access delay register (SPND)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPNDEN_A::_1
    }
}
#[doc = "Field `SPNDEN` writer - RSPI Next-Access Delay Enable"]
pub type SPNDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPNDEN_A>;
impl<'a, REG, const O: u8> SPNDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A next-access delay of 1 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDEN_A::_0)
    }
    #[doc = "A next-access delay is equal to the setting of the RSPI next-access delay register (SPND)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDEN_A::_1)
    }
}
#[doc = "Field `SLNDEN` reader - SSL Negation Delay Setting Enable"]
pub type SLNDEN_R = crate::BitReader<SLNDEN_A>;
#[doc = "SSL Negation Delay Setting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLNDEN_A {
    #[doc = "0: An SSL negation delay of 1 RSPCK"]
    _0 = 0,
    #[doc = "1: An SSL negation delay is equal to the setting of the RSPI slave select negation delay register (SSLND)"]
    _1 = 1,
}
impl From<SLNDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLNDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SLNDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLNDEN_A {
        match self.bits {
            false => SLNDEN_A::_0,
            true => SLNDEN_A::_1,
        }
    }
    #[doc = "An SSL negation delay of 1 RSPCK"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLNDEN_A::_0
    }
    #[doc = "An SSL negation delay is equal to the setting of the RSPI slave select negation delay register (SSLND)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLNDEN_A::_1
    }
}
#[doc = "Field `SLNDEN` writer - SSL Negation Delay Setting Enable"]
pub type SLNDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLNDEN_A>;
impl<'a, REG, const O: u8> SLNDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An SSL negation delay of 1 RSPCK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDEN_A::_0)
    }
    #[doc = "An SSL negation delay is equal to the setting of the RSPI slave select negation delay register (SSLND)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDEN_A::_1)
    }
}
#[doc = "Field `SCKDEN` reader - RSPCK Delay Setting Enable"]
pub type SCKDEN_R = crate::BitReader<SCKDEN_A>;
#[doc = "RSPCK Delay Setting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKDEN_A {
    #[doc = "0: An RSPCK delay of 1 RSPCK"]
    _0 = 0,
    #[doc = "1: An RSPCK delay is equal to the setting of the RSPI clock delay register (SPCKD)"]
    _1 = 1,
}
impl From<SCKDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCKDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCKDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCKDEN_A {
        match self.bits {
            false => SCKDEN_A::_0,
            true => SCKDEN_A::_1,
        }
    }
    #[doc = "An RSPCK delay of 1 RSPCK"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCKDEN_A::_0
    }
    #[doc = "An RSPCK delay is equal to the setting of the RSPI clock delay register (SPCKD)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCKDEN_A::_1
    }
}
#[doc = "Field `SCKDEN` writer - RSPCK Delay Setting Enable"]
pub type SCKDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SCKDEN_A>;
impl<'a, REG, const O: u8> SCKDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An RSPCK delay of 1 RSPCK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDEN_A::_0)
    }
    #[doc = "An RSPCK delay is equal to the setting of the RSPI clock delay register (SPCKD)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RSPCK Phase Setting"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RSPCK Polarity Setting"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Bit Rate Division Setting"]
    #[inline(always)]
    pub fn brdv(&self) -> BRDV_R {
        BRDV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - SSL Signal Assertion Setting"]
    #[inline(always)]
    pub fn ssla(&self) -> SSLA_R {
        SSLA_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - RSPI Data Length Setting"]
    #[inline(always)]
    pub fn spb(&self) -> SPB_R {
        SPB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - RSPI LSB First"]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RSPI Next-Access Delay Enable"]
    #[inline(always)]
    pub fn spnden(&self) -> SPNDEN_R {
        SPNDEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SSL Negation Delay Setting Enable"]
    #[inline(always)]
    pub fn slnden(&self) -> SLNDEN_R {
        SLNDEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RSPCK Delay Setting Enable"]
    #[inline(always)]
    pub fn sckden(&self) -> SCKDEN_R {
        SCKDEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RSPCK Phase Setting"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<SPCMD0_SPEC, 0> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 1 - RSPCK Polarity Setting"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<SPCMD0_SPEC, 1> {
        CPOL_W::new(self)
    }
    #[doc = "Bits 2:3 - Bit Rate Division Setting"]
    #[inline(always)]
    #[must_use]
    pub fn brdv(&mut self) -> BRDV_W<SPCMD0_SPEC, 2> {
        BRDV_W::new(self)
    }
    #[doc = "Bits 4:6 - SSL Signal Assertion Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ssla(&mut self) -> SSLA_W<SPCMD0_SPEC, 4> {
        SSLA_W::new(self)
    }
    #[doc = "Bits 8:11 - RSPI Data Length Setting"]
    #[inline(always)]
    #[must_use]
    pub fn spb(&mut self) -> SPB_W<SPCMD0_SPEC, 8> {
        SPB_W::new(self)
    }
    #[doc = "Bit 12 - RSPI LSB First"]
    #[inline(always)]
    #[must_use]
    pub fn lsbf(&mut self) -> LSBF_W<SPCMD0_SPEC, 12> {
        LSBF_W::new(self)
    }
    #[doc = "Bit 13 - RSPI Next-Access Delay Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spnden(&mut self) -> SPNDEN_W<SPCMD0_SPEC, 13> {
        SPNDEN_W::new(self)
    }
    #[doc = "Bit 14 - SSL Negation Delay Setting Enable"]
    #[inline(always)]
    #[must_use]
    pub fn slnden(&mut self) -> SLNDEN_W<SPCMD0_SPEC, 14> {
        SLNDEN_W::new(self)
    }
    #[doc = "Bit 15 - RSPCK Delay Setting Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sckden(&mut self) -> SCKDEN_W<SPCMD0_SPEC, 15> {
        SCKDEN_W::new(self)
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
#[doc = "SPI Command Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spcmd0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spcmd0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPCMD0_SPEC;
impl crate::RegisterSpec for SPCMD0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spcmd0::R`](R) reader structure"]
impl crate::Readable for SPCMD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spcmd0::W`](W) writer structure"]
impl crate::Writable for SPCMD0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCMD0 to value 0x070d"]
impl crate::Resettable for SPCMD0_SPEC {
    const RESET_VALUE: Self::Ux = 0x070d;
}
