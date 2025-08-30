import ts from 'typescript'
import MagicString from 'magic-string'
import path from 'path'
import process from 'process'

// In-memory file system for the language service
const files = new Map()

/**
 * @returns {{ script: import('svelte/compiler').Preprocessor }}
 */
function script() {
	return {
		async script({ attributes, content, filename = '' }) {
			if (
				attributes.lang !== 'ts' ||
				attributes.context === 'module' ||
				filename.includes('/node_modules/')
			) {
				return
			}

			const resolvedFilename = path.resolve(filename)

			// This is a test harness specific to the user's code, so we can keep it
			if (resolvedFilename !== '/Users/k/git/kadium/src/lib/Button.svelte') {
				return
			}

			console.log(`\n\n\n\n ${resolvedFilename} ::::::::::::::::::`, attributes)
			console.log(content)

			// FIX: Replace 'export let' with 'let' to create a valid TypeScript snippet for the type checker.
			const tsContent = content.replace(/export\s+let/g, 'let')

			// Add the Svelte file content to the in-memory file system, but under a .ts extension
			// so the TypeScript language service can properly process it.
			const tsFilename = resolvedFilename + '.ts'
			files.set(tsFilename, tsContent)

			// Set up the TypeScript language service and program
			const servicesHost = {
				getScriptFileNames: () => [tsFilename],
				getScriptVersion: (fileName) => '1',
				getScriptSnapshot: (fileName) => {
					// First, check the in-memory files.
					if (files.has(fileName)) {
						return ts.ScriptSnapshot.fromString(files.get(fileName))
					}
					// If not in-memory, read from the real file system.
					const fileContent = ts.sys.readFile(fileName)
					if (fileContent) {
						return ts.ScriptSnapshot.fromString(fileContent)
					}
					return undefined
				},
				getCurrentDirectory: () => process.cwd(),
				getCompilationSettings: () => ({
					target: ts.ScriptTarget.Latest,
					module: ts.ModuleKind.ESNext,
					jsx: ts.JsxEmit.Preserve,
				}),
				getDefaultLibFileName: (options) => ts.getDefaultLibFilePath(options),
				fileExists: ts.sys.fileExists,
				readFile: ts.sys.readFile,
				readDirectory: ts.sys.readDirectory,
				getNewLine: () => ts.sys.newLine,
			}

			const services = ts.createLanguageService(servicesHost, ts.createDocumentRegistry())
			const program = services.getProgram()
			if (!program) {
				console.log('(no program)')
				return
			}

			// FIX: Get the source file using the temporary .ts filename.
			const source_file = program.getSourceFile(tsFilename)
			if (!source_file) {
				console.log('(no source_file)')
				return
			}
			const checker = program.getTypeChecker()

			/** @type { name: string; type?: string; initializer?: string }[] */
			const exports = []

			// collect all `let` declarations from the stripped TS content
			source_file.forEachChild((node) => {
				if (ts.isVariableStatement(node)) {
					// Since we removed 'export', we now just look for 'let'
					// We can also check for variable declarations without modifiers, as 'export' has been removed
					const hasLetKeyword = node.declarationList.flags & ts.NodeFlags.Let
					if (!hasLetKeyword) return

					node.declarationList.declarations.forEach((decl) => {
						if (!ts.isIdentifier(decl.name)) return

						const name = decl.name.text
						const initializer = decl.initializer ? decl.initializer.getText(source_file) : undefined

						// Use the type checker to get the type of the variable
						const symbol = checker.getSymbolAtLocation(decl.name)
						if (!symbol) return

						const type = checker.getTypeOfSymbolAtLocation(symbol, decl.name)
						const typeString = checker.typeToString(
							type,
							source_file,
							ts.TypeFormatFlags.NoTruncation,
						)

						exports.push({ name, type: typeString, initializer })
					})
				}
			})

			// if no props, do nothing
			if (!exports.length) {
				console.log('(No props found)')
				return
			}

			const s = new MagicString(content)

			// remove original export let lines
			// FIX: Use the original source content to find and remove the lines.
			const originalSourceFile = ts.createSourceFile(
				resolvedFilename,
				content,
				ts.ScriptTarget.Latest,
				true,
				ts.ScriptKind.TSX,
			)

			for (const node of originalSourceFile.statements) {
				if (ts.isVariableStatement(node)) {
					const isExport = node.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword)
					if (isExport) {
						s.remove(node.pos, node.end)
					}
				}
			}

			// construct the new destructure
			let prop_content = ''
			for (const e of exports) {
				prop_content += `\t${e.name}${e.initializer ? ` = ${e.initializer}` : ''},\n`
			}
			let type_content = ''
			for (const e of exports) {
				type_content += `\t${e.name}${e.type ? `: ${e.type}` : ''},\n`
			}
			const destructure = `let {\n${prop_content}}: {\n${type_content}} = $props();\n`

			// prepend to <script>
			s.prepend(destructure)

			console.log('\n ----------->\n')
			console.log(s.toString())

			return { code: s.toString(), map: s.generateMap({ hires: true, source: filename }) }
		},
	}
}

/**
 * @returns {import('svelte/compiler').PreprocessorGroup}
 */
export function props_preprocess() {
	/** @type {import('svelte/compiler').PreprocessorGroup} */
	const preprocessor = { name: 'vite-preprocess', script: script().script }
	return preprocessor
}
