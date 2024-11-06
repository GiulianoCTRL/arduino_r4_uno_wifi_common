#[doc = "Register `FCR` reader"]
pub type R = crate::R<FCR_SPEC>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCR_SPEC>;
#[doc = "Field `FM` reader - FIFO Mode Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type FM_R = crate::BitReader<FM_A>;
#[doc = "FIFO Mode Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FM_A {
    #[doc = "0: Non-FIFO mode(Selects o TDR/RDR for communication)"]
    _0 = 0,
    #[doc = "1: FIFO mode (Selects to FTDRH and FTDRL/FRDRH and FRDRL for communication)"]
    _1 = 1,
}
impl From<FM_A> for bool {
    #[inline(always)]
    fn from(variant: FM_A) -> Self {
        variant as u8 != 0
    }
}
impl FM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FM_A {
        match self.bits {
            false => FM_A::_0,
            true => FM_A::_1,
        }
    }
    #[doc = "Non-FIFO mode(Selects o TDR/RDR for communication)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FM_A::_0
    }
    #[doc = "FIFO mode (Selects to FTDRH and FTDRL/FRDRH and FRDRL for communication)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FM_A::_1
    }
}
#[doc = "Field `FM` writer - FIFO Mode Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type FM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FM_A>;
impl<'a, REG, const O: u8> FM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-FIFO mode(Selects o TDR/RDR for communication)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FM_A::_0)
    }
    #[doc = "FIFO mode (Selects to FTDRH and FTDRL/FRDRH and FRDRL for communication)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FM_A::_1)
    }
}
#[doc = "Field `RFRST` reader - Receive FIFO Data Register Reset (Valid only in FCR.FM=1)"]
pub type RFRST_R = crate::BitReader<RFRST_A>;
#[doc = "Receive FIFO Data Register Reset (Valid only in FCR.FM=1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRST_A {
    #[doc = "0: The number of data stored in FRDRH and FRDRL register are NOT made 0"]
    _0 = 0,
    #[doc = "1: The number of data stored in FRDRH and FRDRL register are made 0"]
    _1 = 1,
}
impl From<RFRST_A> for bool {
    #[inline(always)]
    fn from(variant: RFRST_A) -> Self {
        variant as u8 != 0
    }
}
impl RFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFRST_A {
        match self.bits {
            false => RFRST_A::_0,
            true => RFRST_A::_1,
        }
    }
    #[doc = "The number of data stored in FRDRH and FRDRL register are NOT made 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFRST_A::_0
    }
    #[doc = "The number of data stored in FRDRH and FRDRL register are made 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFRST_A::_1
    }
}
#[doc = "Field `RFRST` writer - Receive FIFO Data Register Reset (Valid only in FCR.FM=1)"]
pub type RFRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFRST_A>;
impl<'a, REG, const O: u8> RFRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of data stored in FRDRH and FRDRL register are NOT made 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RFRST_A::_0)
    }
    #[doc = "The number of data stored in FRDRH and FRDRL register are made 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RFRST_A::_1)
    }
}
#[doc = "Field `TFRST` reader - Transmit FIFO Data Register Reset (Valid only in FCR.FM=1)"]
pub type TFRST_R = crate::BitReader<TFRST_A>;
#[doc = "Transmit FIFO Data Register Reset (Valid only in FCR.FM=1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFRST_A {
    #[doc = "0: The number of data stored in FTDRH and FTDRL register are NOT made 0"]
    _0 = 0,
    #[doc = "1: The number of data stored in FTDRH and FTDRL register are made 0"]
    _1 = 1,
}
impl From<TFRST_A> for bool {
    #[inline(always)]
    fn from(variant: TFRST_A) -> Self {
        variant as u8 != 0
    }
}
impl TFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFRST_A {
        match self.bits {
            false => TFRST_A::_0,
            true => TFRST_A::_1,
        }
    }
    #[doc = "The number of data stored in FTDRH and FTDRL register are NOT made 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFRST_A::_0
    }
    #[doc = "The number of data stored in FTDRH and FTDRL register are made 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFRST_A::_1
    }
}
#[doc = "Field `TFRST` writer - Transmit FIFO Data Register Reset (Valid only in FCR.FM=1)"]
pub type TFRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TFRST_A>;
impl<'a, REG, const O: u8> TFRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The number of data stored in FTDRH and FTDRL register are NOT made 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TFRST_A::_0)
    }
    #[doc = "The number of data stored in FTDRH and FTDRL register are made 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TFRST_A::_1)
    }
}
#[doc = "Field `DRES` reader - Receive data ready error select bit (When detecting a reception data ready, the interrupt request is selected.)"]
pub type DRES_R = crate::BitReader<DRES_A>;
#[doc = "Receive data ready error select bit (When detecting a reception data ready, the interrupt request is selected.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRES_A {
    #[doc = "0: reception data full interrupt (RXI)"]
    _0 = 0,
    #[doc = "1: receive error interrupt (ERI)"]
    _1 = 1,
}
impl From<DRES_A> for bool {
    #[inline(always)]
    fn from(variant: DRES_A) -> Self {
        variant as u8 != 0
    }
}
impl DRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRES_A {
        match self.bits {
            false => DRES_A::_0,
            true => DRES_A::_1,
        }
    }
    #[doc = "reception data full interrupt (RXI)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRES_A::_0
    }
    #[doc = "receive error interrupt (ERI)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRES_A::_1
    }
}
#[doc = "Field `DRES` writer - Receive data ready error select bit (When detecting a reception data ready, the interrupt request is selected.)"]
pub type DRES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DRES_A>;
impl<'a, REG, const O: u8> DRES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "reception data full interrupt (RXI)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRES_A::_0)
    }
    #[doc = "receive error interrupt (ERI)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRES_A::_1)
    }
}
#[doc = "Field `TTRG` reader - Transmit FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type TTRG_R = crate::FieldReader<TTRG_A>;
#[doc = "Transmit FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TTRG_A {
    #[doc = "0: Trigger number 0"]
    _0000 = 0,
}
impl From<TTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: TTRG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TTRG_A {
    type Ux = u8;
}
impl TTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TTRG_A> {
        match self.bits {
            0 => Some(TTRG_A::_0000),
            _ => None,
        }
    }
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TTRG_A::_0000
    }
}
#[doc = "Field `TTRG` writer - Transmit FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type TTRG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, TTRG_A>;
impl<'a, REG, const O: u8> TTRG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(TTRG_A::_0000)
    }
}
#[doc = "Field `RTRG` reader - Receive FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RTRG_R = crate::FieldReader<RTRG_A>;
#[doc = "Receive FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTRG_A {
    #[doc = "0: Trigger number 0"]
    _0000 = 0,
}
impl From<RTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: RTRG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTRG_A {
    type Ux = u8;
}
impl RTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RTRG_A> {
        match self.bits {
            0 => Some(RTRG_A::_0000),
            _ => None,
        }
    }
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == RTRG_A::_0000
    }
}
#[doc = "Field `RTRG` writer - Receive FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RTRG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, RTRG_A>;
impl<'a, REG, const O: u8> RTRG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(RTRG_A::_0000)
    }
}
#[doc = "Field `RSTRG` reader - RTS Output Active Trigger Number Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RSTRG_R = crate::FieldReader<RSTRG_A>;
#[doc = "RTS Output Active Trigger Number Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSTRG_A {
    #[doc = "0: Trigger number 0"]
    _0000 = 0,
}
impl From<RSTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTRG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSTRG_A {
    type Ux = u8;
}
impl RSTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RSTRG_A> {
        match self.bits {
            0 => Some(RSTRG_A::_0000),
            _ => None,
        }
    }
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == RSTRG_A::_0000
    }
}
#[doc = "Field `RSTRG` writer - RTS Output Active Trigger Number Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
pub type RSTRG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, RSTRG_A>;
impl<'a, REG, const O: u8> RSTRG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger number 0"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(RSTRG_A::_0000)
    }
}
impl R {
    #[doc = "Bit 0 - FIFO Mode Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn fm(&self) -> FM_R {
        FM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO Data Register Reset (Valid only in FCR.FM=1)"]
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Data Register Reset (Valid only in FCR.FM=1)"]
    #[inline(always)]
    pub fn tfrst(&self) -> TFRST_R {
        TFRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive data ready error select bit (When detecting a reception data ready, the interrupt request is selected.)"]
    #[inline(always)]
    pub fn dres(&self) -> DRES_R {
        DRES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Transmit FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn ttrg(&self) -> TTRG_R {
        TTRG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Receive FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn rtrg(&self) -> RTRG_R {
        RTRG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RTS Output Active Trigger Number Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    pub fn rstrg(&self) -> RSTRG_R {
        RSTRG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Mode Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn fm(&mut self) -> FM_W<FCR_SPEC, 0> {
        FM_W::new(self)
    }
    #[doc = "Bit 1 - Receive FIFO Data Register Reset (Valid only in FCR.FM=1)"]
    #[inline(always)]
    #[must_use]
    pub fn rfrst(&mut self) -> RFRST_W<FCR_SPEC, 1> {
        RFRST_W::new(self)
    }
    #[doc = "Bit 2 - Transmit FIFO Data Register Reset (Valid only in FCR.FM=1)"]
    #[inline(always)]
    #[must_use]
    pub fn tfrst(&mut self) -> TFRST_W<FCR_SPEC, 2> {
        TFRST_W::new(self)
    }
    #[doc = "Bit 3 - Receive data ready error select bit (When detecting a reception data ready, the interrupt request is selected.)"]
    #[inline(always)]
    #[must_use]
    pub fn dres(&mut self) -> DRES_W<FCR_SPEC, 3> {
        DRES_W::new(self)
    }
    #[doc = "Bits 4:7 - Transmit FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ttrg(&mut self) -> TTRG_W<FCR_SPEC, 4> {
        TTRG_W::new(self)
    }
    #[doc = "Bits 8:11 - Receive FIFO data trigger number (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn rtrg(&mut self) -> RTRG_W<FCR_SPEC, 8> {
        RTRG_W::new(self)
    }
    #[doc = "Bits 12:15 - RTS Output Active Trigger Number Select (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn rstrg(&mut self) -> RSTRG_W<FCR_SPEC, 12> {
        RSTRG_W::new(self)
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
#[doc = "FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR to value 0xf800"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xf800;
}
