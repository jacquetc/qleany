// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "undo_redo/query_command.h"

using namespace FrontEnds::Controller::UndoRedo;

/*!
 * \brief Constructs a QueryCommand instance with the specified text for logging purposes.
 * \param text A QString representing the message for logging purposes.
 */
QueryCommand::QueryCommand(const QString &text)
    : UndoRedoCommand(text)
{
    this->setType(Type::QueryCommand);
}

/*!
 * \brief Sets the function to be executed asynchronously when "redo()" is called in UndoRedoSystem.
 * \param function A std::function<void()> representing the function to be executed.
 */
void QueryCommand::setQueryFunction(const std::function<Result<void>(QPromise<Result<void>> &promise)> &function)
{
    m_queryFunction = function;
    this->setRedoFunction([this](QPromise<Result<void>> &progressPromise) {
        return Result<void>(m_queryFunction(progressPromise));
    });
}