//! Abstractions over the UART registers of the Tegra X1.
//!
//! See Chapter 36.3 in the Tegra X1 Technical Reference Manual for details.

use tock_registers::{register_bitfields, register_structs, registers::*};

register_bitfields! {
    u32,

    /// Bitfields of the `UART_THR_DLAB_0_0` register.
    pub UART_THR_DLAB_0_0 [
        /// The Transmit Holding Register.
        ///
        /// It holds the characters to be transmitted by the UART.
        /// In FIFO mode, a write to this FIFO places the data at
        /// the end of the FIFO.
        ///
        /// NOTE: These bits are write-only.
        THR_A OFFSET(0) NUMBITS(8) [],

        /// The Receive Buffer Register.
        ///
        /// Rx data can be read from here.
        ///
        /// NOTE: These bits are read-only.
        RBR_A OFFSET(0) NUMBITS(8) [],

        /// The Divisor Latch LSB register.
        ///
        /// The value is represented by the low 8 bits of the 16-bit
        /// Baud Divisor.
        ///
        /// NOTE: These bits are read-only.
        DLL_A OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `UART_IER_DLAB_0_0` register.
    pub UART_IER_DLAB_0_0 [
        /// Interrupt Enable for End of Received Data.
        IE_EORD OFFSET(5) NUMBITS(1) [],

        /// Interrupt Enable for Rx FIFO timeout.
        IE_RX_TIMEOUT OFFSET(4) NUMBITS(1) [],

        /// Interrupt Enable for Modem Status Interrupt.
        IE_MSI OFFSET(3) NUMBITS(1) [],

        /// Interrupt Enable for Receiver Line Status Interrupt.
        IE_RXS OFFSET(2) NUMBITS(1) [],

        /// Interrupt Enable for Transmitter Holding Register Empty Interrupt.
        IE_THR OFFSET(1) NUMBITS(1) [],

        /// Interrupt Enable for Receive Data Interrupt.
        IE_RHR OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `UART_IIR_FCR_0` register.
    pub UART_IIR_FCR_0 [
        /// FIFO Mode Status.
        EN_FIFO OFFSET(6) NUMBITS(2) [
            /// 16450 Mode.
            ///
            /// This mode disables FIFOs.
            Mode16450 = 0,
            /// 16550 Mode.
            ///
            /// This mode enables FIFOs.
            Mode16550 = 1
        ],

        RX_TRIG OFFSET(6) NUMBITS(2) [
            FifoCountGreater1 = 0,
            FifoCountGreater4 = 1,
            FifoCountGreater8 = 2,
            FifoCountGreater16 = 3
        ],

        TX_TRIG OFFSET(4) NUMBITS(2) [
            FifoCountGreater16 = 0,
            FifoCountGreater8 = 1,
            FifoCountGreater4 = 2,
            FifoCountGreater1 = 3
        ],

        /// Whether Encoded Interrupt IDs should be enabled or not.
        IS_PRI2 OFFSET(3) NUMBITS(1) [],

        /// The DMA mode to use.
        DMA OFFSET(3) NUMBITS(1) [
            /// DMA Mode 0.
            ///
            /// This is the default mode.
            DmaMode0 = 0,
            /// DMA Mode 1.
            DmaMode1 = 1
        ],

        /// Whether Encoded Interrupt IDs should be enabled or not.
        IS_PRI1 OFFSET(2) NUMBITS(1) [],

        /// Clears the contents of the transmit FIFO and resets its counter logic to 0.
        TX_CLR OFFSET(2) NUMBITS(1) [
            /// Indicates that the FIFOs were cleared.
            NoClear = 0,
            /// Indicates that the FIFOs should be cleared or are being cleared right now.
            Clear = 1
        ],

        /// Whether Encoded Interrupt IDs should be enabled or not.
        IS_PRI0 OFFSET(1) NUMBITS(1) [],

        /// Clears the contents of the receive FIFO and resets the counter logic to 0.
        RX_CLR OFFSET(1) NUMBITS(1) [
            /// Indicates that the FIFOs were cleared.
            NoClear = 0,
            /// Indicates that the FIFOs should be cleared or are being cleared right now.
            Clear = 1
        ],

        /// Checks whether an interrupt is pending.
        IS_STA OFFSET(0) NUMBITS(1) [
            /// An interrupt is pending.
            IntrPend = 0,
            /// No interrupt is pending.
            NoIntrPend = 1
        ],

        /// Enables the transmit and receive FIFOs.
        ///
        /// This bit should always be enabled.
        FCR_EN_FIFO OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `UART_LCR_0` register.
    pub UART_LCR_0 [
        /// Whether the Divisor Latch Access Bit should be enabled.
        ///
        /// NOTE: Set this bit to allow programming of the DLH and DLM Divisors.
        DLAB OFFSET(7) NUMBITS(1) [],

        /// Whether a BREAK condition should be set.
        ///
        /// NOTE: The transmitter sends all zeroes to indicate a BREAK.
        SET_B OFFSET(6) NUMBITS(1) [],

        /// Whether parity should be set (forced) to the value in LCR.
        SET_P OFFSET(5) NUMBITS(1) [],

        /// Whether the even parity format should be used for number representation.
        ///
        /// NOTE: There will always be an even number of 1s in the binary representation.
        EVEN OFFSET(4) NUMBITS(1) [],

        /// Whether parity should be sent or not.
        PAR OFFSET(3) NUMBITS(1) [],

        /// Whether 2 stop bits should be transmitted instead of 1.
        ///
        /// NOTE: The receiver always checks for 1 stop bit.
        STOP OFFSET(2) NUMBITS(1) [],

        /// The Word Length size.
        WD_SIZE OFFSET(0) NUMBITS(2) [
            /// Word length of 5.
            WordLength5 = 0,
            /// Word length of 6.
            WordLength6 = 1,
            /// Word length of 7.
            WordLength7 = 2,
            /// Word length of 8.
            WordLength8 = 3
        ]
    ],

    /// Bitfields of the `UART_MCR_0` register.
    pub UART_MCR_0 [
        /// Whether the old SIR decode path should be used instead of the new one.
        OLD_SIR_DECODE OFFSET(10) NUMBITS(1) [],

        /// Polarity selection bit for RI pin toggling to generate model status interrupt.
        RI_POLARITY OFFSET(8) NUMBITS(2) [
            /// Interrupt will be generated when RI pin toggles from low to high.
            LowToHigh = 0,
            /// Interrupt will be generated when RI pin toggles from high to low.
            HighToLow = 1,
            /// Interrupt will be generated on RI delta change detection.
            BothEdges = 2,
            /// Reserved.
            Reserved = 3
        ],

        /// Whether the old qualified CTS in TX state machine should be used.
        DEL_QUAL_CTS_EN OFFSET(7) NUMBITS(1) [],

        /// Whether RTS Hardware Flow Control should be enabled.
        RTS_EN OFFSET(6) NUMBITS(1) [],

        /// Whether CTS Hardware Flow Control should be enabled.
        CTS_EN OFFSET(5) NUMBITS(1) [],

        /// Whether internal loop back of Serial Out to In should be enabled.
        LOOPBK OFFSET(4) NUMBITS(1) [],

        /// nOUT2 (Not Used).
        OUT2 OFFSET(3) NUMBITS(1) [],

        /// nOUT1 (Not Used).
        OUT1 OFFSET(2) NUMBITS(1) [],

        /// Whether RTS should be forced to high if RTS hardware flow control wasn't enabled.
        RTS OFFSET(1) NUMBITS(1) [],

        /// Whether DTR should be forced to high or not.
        DTR OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `UART_LSR_0` register.
    pub UART_LSR_0 [
        /// Whether the RX FIFO is empty.
        RX_FIFO_EMPTY OFFSET(9) NUMBITS(1) [],

        /// Whether the Transmitter FIFO is full.
        TX_FIFO_FULL OFFSET(8) NUMBITS(1) [],

        /// Denotes a Receive FIFO error, if set to 1.
        FIFOE OFFSET(7) NUMBITS(1) [],

        /// Denotes a Transmit Shift Register empty status, if set to 1.
        TMTY OFFSET(6) NUMBITS(1) [],

        /// Denotes that the Transmit Holding Register is empty, if set to 1.
        ///
        /// This means that data can be written.
        THRE OFFSET(5) NUMBITS(1) [],

        /// Denotes that a BREAK condition was detected on the line, if set to 1.
        BRK OFFSET(4) NUMBITS(1) [],

        /// Denotes a Framing Error, if set to 1.
        FERR OFFSET(3) NUMBITS(1) [],

        /// Denotes a Parity Error, if set to 1.
        PERR OFFSET(2) NUMBITS(1) [],

        /// Denotes a Receiver Overrun Error, if set to 1.
        OVRF OFFSET(1) NUMBITS(1) [],

        /// Denotes that Receiver Data are in FIFO, if set to 1.
        ///
        /// This means that data are available to read.
        RDR OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `UART_MSR_0` register.
    pub UART_MSR_0 [
        /// State of Carrier detect pin.
        CD OFFSET(7) NUMBITS(1) [],

        /// State of Ring Indicator pin.
        RI OFFSET(6) NUMBITS(1) [],

        /// State of Data set ready pin.
        DSR OFFSET(5) NUMBITS(1) [],

        /// State of Clear to send pin.
        CTS OFFSET(4) NUMBITS(1) [],

        /// Change (Delta) in CD state detected.
        DCD OFFSET(3) NUMBITS(1) [],

        /// Change (Delta) in RI state detected.
        DRI OFFSET(2) NUMBITS(1) [],

        /// Change (Delta) in DSR state detected.
        DDSR OFFSET(1) NUMBITS(1) [],

        /// Change (Delta) in CTS detected.
        DCTS OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `UART_SPR_0` register.
    pub UART_SPR_0 [
        /// Scratchpad register (not used internally).
        SPR_A OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `UART_IRDA_CSR_0` register.
    pub UART_IRDA_CSR_0 [
        /// Whether SIR coder should be enabled.
        SIR_A OFFSET(7) NUMBITS(1) [],

        /// Controls the Baud Pulse.
        PWT_A OFFSET(6) NUMBITS(1) [
            /// 3/16th Baud Pulse.
            BaudPulse314 = 0,
            /// 4/16th Baud Pulse.
            BaudPulse414 = 1
        ],

        /// Inverts the normally inactive high nRTS pin.
        INVERT_RTS OFFSET(3) NUMBITS(1) [],

        /// Inverts the normally inactive high nCTS pin.
        INVERT_CTS OFFSET(2) NUMBITS(1) [],

        /// Inverts the normally inactive high TXD pin.
        INVERT_TXD OFFSET(1) NUMBITS(1) [],

        /// Inverts the normally inactive high RXD pin.
        INVERT_RXD OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `UART_RX_FIFO_CFG_0` register.
    pub UART_RX_FIFO_CFG_0 [
        /// Enables the use of `RX_FIFO_TRIG` count, if set to 1.
        ///
        /// This obsoletes `RX_TRIG` when enabled.
        EN_RX_FIFO_TRIG OFFSET(7) NUMBITS(1) [],

        /// Set RX_FIFO trigger level.
        ///
        /// This can be any value from 1 through 32.
        RX_FIFO_TRIG OFFSET(0) NUMBITS(6) []
    ],

    /// Bitfields of the `UART_MIE_0` register.
    pub UART_MIE_0 [
        /// Interrupt Enable for Change (Delta) in CD state detected.
        DCD_INT_EN OFFSET(3) NUMBITS(1) [],

        /// Interrupt Enable for Change (Delta) in RI state detected.
        DRI_INT_EN OFFSET(2) NUMBITS(1) [],

        /// Interrupt Enable for Change (Delta) in DSR state detected.
        DDSR_INT_EN OFFSET(1) NUMBITS(1) [],

        /// Interrupt Enable for Change (Delta) in CTS state detected.
        DCTS_INT_EN OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `UART_VENDOR_STATUS_0_0` register.
    pub UART_VENDOR_STATUS_0_0 [
        /// The entry in this field reflects the number of current entries in the TX FIFO.
        TX_FIFO_COUNTER OFFSET(24) NUMBITS(6) [],

        /// The entry in this field reflects the number of current entries in the RX FIFO.
        RX_FIFO_COUNTER OFFSET(16) NUMBITS(6) [],

        /// This bit is set to 1 when write data is issued to the TX FIFO when it
        /// is already full and gets cleared on register read (sticky bit until read).
        TX_OVERRUN OFFSET(3) NUMBITS(1) [],

        /// This bit is set to 1 when a read is issued to an empty FIFO and gets
        /// cleared on register read (sticky bit until read).
        RX_UNDERRUN OFFSET(2) NUMBITS(1) [],

        /// This bit is set to 1 when the RX path is IDLE.
        UART_RX_IDLE OFFSET(1) NUMBITS(1) [
            /// The path is busy.
            Busy = 0,
            /// The path is in idle.
            Idle = 1
        ],

        /// This bit is set to 1 when the TX path is IDLE.
        UART_TX_IDLE OFFSET(0) NUMBITS(1) [
            /// The path is busy.
            Busy = 0,
            /// The path is in idle.
            Idle = 1
        ]
    ],

    /// Bitfields of the `UART_ASR_0` register.
    pub UART_ASR_0 [
        /// This bit is set when the controller finishes counting the clocks between two
        /// successive clock edges after there is a write to ASR with don't care data.
        VALID OFFSET(31) NUMBITS(1) [],

        /// This bit is set when there is a write to ASR and is reset when the controller
        /// finishes counting the clock edges between two successive clock edges.
        BUSY OFFSET(30) NUMBITS(1) [],

        /// Shows bits `[15:8]` of the count of clock edges between two successive clock edges.
        RX_RATE_SENSE_H OFFSET(8) NUMBITS(8) [],

        /// Shows bits `[7:0]` of the count of clock edges between two successive clock edges.
        RX_RATE_SENSE_L OFFSET(0) NUMBITS(7) []
    ]
}

register_structs! {
    /// Representation of the UART registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => pub UART_THR_DLAB_0_0: ReadWrite<u32, UART_THR_DLAB_0_0::Register>),
        (0x04 => pub UART_IER_DLAB_0_0: ReadWrite<u32, UART_IER_DLAB_0_0::Register>),
        (0x08 => pub UART_IIR_FCR_0: ReadWrite<u32, UART_IIR_FCR_0::Register>),
        (0x0C => pub UART_LCR_0: ReadWrite<u32, UART_LCR_0::Register>),
        (0x10 => pub UART_MCR_0: ReadWrite<u32, UART_MCR_0::Register>),
        (0x14 => pub UART_LSR_0: ReadOnly<u32, UART_LSR_0::Register>),
        (0x18 => pub UART_MSR_0: ReadWrite<u32, UART_MSR_0::Register>),
        (0x1C => pub UART_SPR_0: ReadWrite<u32, UART_SPR_0::Register>),
        (0x20 => pub UART_IRDA_CSR_0: ReadWrite<u32, UART_IRDA_CSR_0::Register>),
        (0x24 => pub UART_RX_FIFO_CFG_0: ReadWrite<u32, UART_RX_FIFO_CFG_0::Register>),
        (0x28 => pub UART_MIE_0: ReadWrite<u32, UART_MIE_0::Register>),
        (0x2C => pub UART_VENDOR_STATUS_0_0: ReadOnly<u32, UART_VENDOR_STATUS_0_0::Register>),
        (0x30 => _reserved),
        (0x3C => pub UART_ASR_0: ReadWrite<u32, UART_ASR_0::Register>),
        (0x40 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x40]);
