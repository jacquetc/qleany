// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#pragma once

#include "repository/interface_repository_provider.h"
#include "simple_example_controller_export.h"
#include <QObject>

namespace Simple::Controller
{

class SIMPLE_EXAMPLE_CONTROLLER_EXPORT ControllerRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit ControllerRegistration(QObject *parent,
                                    Simple::Contracts::Repository::InterfaceRepositoryProvider *repositoryProvider);
    ~ControllerRegistration();

  private:
};

} // namespace Simple::Controller