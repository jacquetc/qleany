#pragma once

#include "error.h"
#include "simple_example_controller_export.h"
#include "undo_redo/undo_redo_command.h"
#include <QObject>

namespace Simple::Controller
{

class SIMPLE_EXAMPLE_CONTROLLER_EXPORT ProgressSignals : public QObject
{
    Q_OBJECT
  public:
    explicit ProgressSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

    void bindToProgressSignals(Simple::Controller::UndoRedo::UndoRedoCommand *undoRedoCommand)
    {
        QObject::connect(undoRedoCommand, &Simple::Controller::UndoRedo::UndoRedoCommand::progressStarted, this,
                         &ProgressSignals::progressStarted, Qt::QueuedConnection);
        QObject::connect(undoRedoCommand, &Simple::Controller::UndoRedo::UndoRedoCommand::progressFinished, this,
                         &ProgressSignals::progressFinished, Qt::QueuedConnection);
        QObject::connect(undoRedoCommand, &Simple::Controller::UndoRedo::UndoRedoCommand::progressRangeChanged, this,
                         &ProgressSignals::progressRangeChanged, Qt::QueuedConnection);
        QObject::connect(undoRedoCommand, &Simple::Controller::UndoRedo::UndoRedoCommand::progressTextChanged, this,
                         &ProgressSignals::progressTextChanged, Qt::QueuedConnection);
        QObject::connect(undoRedoCommand, &Simple::Controller::UndoRedo::UndoRedoCommand::progressValueChanged, this,
                         &ProgressSignals::progressValueChanged, Qt::QueuedConnection);
    }

  Q_SIGNALS:
    void progressStarted();
    void progressFinished();
    void progressRangeChanged(int minimum, int maximum);
    void progressTextChanged(const QString &progressText);
    void progressValueChanged(int progressValue);
};
} // namespace Simple::Controller