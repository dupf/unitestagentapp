declare namespace Unitest {

	interface UnitestMessage {
		sourcefilePath: string
		testfilePath: string
		testfileOutputPath: string
		codecoveragereportPath: string
		testCommand: string
		testCommandDir: string
		includedFiles: string
		coverageType: string
		reportFilepath: string
		desiredCoverage: number
		maxIterations: number
		additionalInstructions: string
		model: string
	}
	//
	interface Unitest {
		dateTime: string
		unitestMessage: UnitestMessage
		tokenNum?: number
		rule: import("@/store").ChatRule
		error?: boolean
		loading?: boolean
	}
	

	interface History {
		title: string
		isEdit: boolean
		uuid: number
	}

	interface UnitestOptions {
		// apiKey: string
		// proxy: string | null
		apiUrl: string
		model: string
		systemMessage: string
		temperature: number
	}

	// 一个聊天会话
	interface UnitestSession {
		uuid: number 
		data: Unitest[]
		opt: Partial<UnitestOptions>
	}

	interface UnitestState {
		active: number | null
		history: History[]
		unitest: UnitestSession[]
	}

	interface RequestMessage {
		role: string
		content: string
	}

	
}