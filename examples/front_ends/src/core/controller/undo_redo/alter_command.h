// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "undo_redo_command.h"

namespace FrontEnds::Controller::UndoRedo
{
///
/// \brief The AlterCommand class
/// Used for commands doing actions, in opposition to QueryCommand which for
// read-only requests
template<class Handler, class Request>
class AlterCommand : public UndoRedoCommand
{
public:
    AlterCommand(const QString &text, Handler *handler, const Request &request)
        : UndoRedoCommand(text)
        , m_handler(handler)
        , m_request(request)
    {
        this->setType(Type::AlterCommand);
        this->setUndoFunction([this]() {
            return Result<void>(m_handler->restore().error());
        });
        this->setRedoFunction([this](QPromise<Result<void>> &progressPromise) {
            m_handler->handle(progressPromise, m_request);
        });
        this->setMergeWithFunction([this](const UndoRedoCommand *other) {
            //            if (other->type() == Type::AlterCommand)
            //            {
            //                const AlterCommand *alterCommand =
            // static_cast<const AlterCommand *>(other);
            //                return
            // m_handler->merge(alterCommand->m_request);
            //            }
            return false;
        });
    }

    // UndoRedoCommand interface

public:
private:
    Handler *m_handler;
    Request m_request;
};
} // namespace FrontEnds::Controller::UndoRedo