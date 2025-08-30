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
				content.includes('$props(') ||
				content.includes('$:') ||
				filename.includes('/node_modules/')
			) {
				return
			}

			const resolvedFilename = path.resolve(filename)

			// Do not remove
			if (resolvedFilename !== '/Users/k/git/kadium/src/lib/Button.svelte') {
				return
			}

			console.log(`\n\n\n\n :::::::::::::::::: ${resolvedFilename}`)
			console.log(content)

			const tsContent = content
			const tsFilename = resolvedFilename + '.ts'
			files.set(tsFilename, tsContent)

			const servicesHost = {
				getScriptFileNames: () => [tsFilename],
				getScriptVersion: () => '1',
				getScriptSnapshot: (fileName) => {
					if (files.has(fileName)) {
						return ts.ScriptSnapshot.fromString(files.get(fileName))
					}
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

			const source_file = program.getSourceFile(tsFilename)
			if (!source_file) {
				console.log('(no source_file)')
				return
			}
			const checker = program.getTypeChecker()

			/** @type { name: string; type?: string; initializer?: string }[] */
			const exports = []

			const originalSourceFile = ts.createSourceFile(
				resolvedFilename,
				content,
				ts.ScriptTarget.Latest,
				true,
				ts.ScriptKind.TSX,
			)
			const originalDecls = new Map()
			originalSourceFile.forEachChild((node) => {
				if (ts.isVariableStatement(node)) {
					node.declarationList.declarations.forEach((decl) => {
						if (ts.isIdentifier(decl.name)) {
							originalDecls.set(decl.name.text, decl)
						}
					})
				}
			})

			source_file.forEachChild((node) => {
				if (
					ts.isVariableStatement(node) &&
					node.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword)
				) {
					node.declarationList.declarations.forEach((decl) => {
						if (!ts.isIdentifier(decl.name)) return

						const name = decl.name.text
						const initializer = decl.initializer ? decl.initializer.getText(source_file) : undefined

						const originalDecl = originalDecls.get(name)
						let typeString

						if (originalDecl && originalDecl.type) {
							// FIX: Always use the explicit type from the original Svelte AST
							typeString = originalDecl.type.getText(originalSourceFile)
						} else {
							const symbol = checker.getSymbolAtLocation(decl.name)
							if (symbol) {
								const type = checker.getTypeOfSymbolAtLocation(symbol, decl.name)
								typeString = checker.typeToString(
									type,
									source_file,
									ts.TypeFormatFlags.NoTruncation,
								)
							}
						}

						if (typeString && typeString !== 'any') {
							exports.push({ name, type: typeString, initializer })
						} else {
							exports.push({ name, initializer })
						}
					})
				}
			})

			if (!exports.length) {
				console.log('(No props found)')
				return
			}

			const s = new MagicString(content)

			// New logic to determine indentation
			const firstStatement = originalSourceFile.statements[0]
			let indent = ''
			if (firstStatement) {
				const start = firstStatement.getStart(originalSourceFile, true)
				const lineStart = content.lastIndexOf('\n', start) + 1
				indent = content.substring(lineStart, start)
			}

			for (const node of originalSourceFile.statements) {
				if (ts.isVariableStatement(node)) {
					const isExport = node.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword)
					if (isExport) {
						s.remove(node.pos, node.end)
					}
				}
			}

			let prop_content = ''
			let type_content = ''
			for (const e of exports) {
				prop_content += `${indent}${indent}${e.name}${e.initializer ? ` = ${e.initializer}` : ''},\n`
				type_content += `${indent}${indent}${e.name}${e.type ? `: ${e.type}` : ''},\n`
			}

			// Prepend the new block with the determined indentation
			const destructure = `${indent}let {\n${prop_content}${indent}}: {\n${type_content}${indent}} = $props();\n`

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
