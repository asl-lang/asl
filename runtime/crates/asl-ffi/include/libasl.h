#ifndef LIBASL_H
#define LIBASL_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Identificador opaco da instância de runtime do ASL
 */
typedef struct asl_runtime_t asl_runtime_t;

/**
 * @brief Identificador opaco de um documento .skill validado e compilado em memória
 */
typedef struct asl_skill_t asl_skill_t;

/**
 * @brief Resultado estruturado da execução determinística in-process
 */
typedef struct {
    uint8_t success;            ///< 1 se executado com sucesso, 0 se ocorreu erro
    const char* json_output;    ///< Ponteiro para string UTF-8 contendo JSON de saída ou erro
    uint64_t fuel_consumed;     ///< Quantidade de opcodes/fuel consumidos
    uint64_t execution_time_ns; ///< Tempo de execução medido em nanossegundos
} asl_exec_result_t;

/**
 * @brief Inicializa uma nova instância do runtime ASL isolado
 * @return Ponteiro para o runtime ou NULL em caso de falha crítica
 */
asl_runtime_t* asl_runtime_init(void);

/**
 * @brief Libera a instância do runtime ASL
 */
void asl_runtime_free(asl_runtime_t* rt);

/**
 * @brief Faz parse e validação de um arquivo ou conteúdo .skill
 * @param rt Instância de runtime ativa
 * @param skill_source String UTF-8 contendo o código-fonte do .skill
 * @return Ponteiro para a skill compilada ou NULL em caso de erro de sintaxe/manifesto
 */
asl_skill_t* asl_skill_load(asl_runtime_t* rt, const char* skill_source);

/**
 * @brief Libera a skill compilada em memória
 */
void asl_skill_free(asl_skill_t* skill);

/**
 * @brief Executa um entrypoint da skill determinística in-process com barreira de pânico
 * @param rt Instância do runtime ativa
 * @param skill Skill compilada
 * @param entrypoint Nome da função (ou NULL para usar o entrypoint padrão do manifesto)
 * @param json_args String UTF-8 contendo JSON com argumentos de entrada (ou NULL para "{}")
 * @return Ponteiro para estrutura asl_exec_result_t alocada na heap
 */
asl_exec_result_t* asl_skill_execute(
    asl_runtime_t* rt,
    asl_skill_t* skill,
    const char* entrypoint,
    const char* json_args
);

/**
 * @brief Libera o resultado da execução e a string JSON associada
 */
void asl_exec_result_free(asl_exec_result_t* result);

#ifdef __cplusplus
}
#endif

#endif /* LIBASL_H */
