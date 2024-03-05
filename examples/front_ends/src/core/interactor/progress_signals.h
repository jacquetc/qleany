#pragma once

#include "front_ends_example_interactor_export.h"
#include <QObject>
#include <qleany/common/error.h>
#include <qleany/tools/undo_redo/undo_redo_command.h>

namespace FrontEnds::Interactor
{

class FRONT_ENDS_EXAMPLE_INTERACTOR_EXPORT ProgressSignals : public QObject
{
    Q_OBJECT
  public:
    explicit ProgressSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

    void bindToProgressSignals(Qleany::Tools::UndoRedo::UndoRedoCommand *undoRedoCommand)
    {
        QObject::connect(undoRedoCommand, &Qleany::Tools::UndoRedo::UndoRedoCommand::progressStarted, this,
                         &ProgressSignals::progressStarted, Qt::QueuedConnection);
        QObject::connect(undoRedoCommand, &Qleany::Tools::UndoRedo::UndoRedoCommand::progressFinished, this,
                         &ProgressSignals::progressFinished, Qt::QueuedConnection);
        QObject::connect(undoRedoCommand, &Qleany::Tools::UndoRedo::UndoRedoCommand::progressRangeChanged, this,
                         &ProgressSignals::progressRangeChanged, Qt::QueuedConnection);
        QObject::connect(undoRedoCommand, &Qleany::Tools::UndoRedo::UndoRedoCommand::progressTextChanged, this,
                         &ProgressSignals::progressTextChanged, Qt::QueuedConnection);
        QObject::connect(undoRedoCommand, &Qleany::Tools::UndoRedo::UndoRedoCommand::progressValueChanged, this,
                         &ProgressSignals::progressValueChanged, Qt::QueuedConnection);
    }

  signals:
    void progressStarted();
    void progressFinished();
    void progressRangeChanged(int minimum, int maximum);
    void progressTextChanged(const QString &progressText);
    void progressValueChanged(int progressValue);
};
} // namespace FrontEnds::Interactor