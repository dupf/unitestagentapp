declare namespace Chat {
	// 一条聊天消息
	interface Chat {
		dateTime: string
		text: string
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

	interface ChatOptions {
		apiKey: string| null
		proxy: string | null
		model: string | null
		systemMessage: string | null
		temperature: number | null
	}

	// 一个聊天会话
	interface ChatSession {
		uuid: number 
		data: Chat[]
		opt: Partial<ChatOptions>
	}

	interface ChatState {
		active: number | null
		history: History[]
		chat: ChatSession[]
	}

	interface RequestMessage {
		role: string
		content: string
	}

//  Unitest  Option 
	interface UnitestOptions {
		// apiKey: string| null
		// proxy: string | null
		model: string | null
		// systemMessage: string | null
		// temperature: number | null
	}

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
}